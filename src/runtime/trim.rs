// Copyright 2026 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use kurbo::{ParamCurve, ParamCurveArclen, PathEl, segments};

const ACCURACY: f64 = 0.1;

pub(crate) fn length(elements: &[PathEl]) -> f64 {
    segments(elements.iter().copied())
        .map(|segment| segment.arclen(ACCURACY))
        .sum()
}

pub(crate) fn trim(
    elements: &[PathEl],
    ranges: &[(f64, f64)],
    sequential: Option<(f64, f64)>,
    output: &mut Vec<PathEl>,
) {
    let mut offset = sequential.map_or(0.0, |(offset, _)| offset);
    let mut start = 0;
    for end in 1..=elements.len() {
        if end != elements.len()
            && !matches!(elements[end], PathEl::MoveTo(_))
            && !matches!(elements[end - 1], PathEl::ClosePath)
        {
            continue;
        }
        let contour = &elements[start..end];
        let contour_length = length(contour);
        let (base, total) = sequential.map_or((0.0, contour_length), |(_, total)| (offset, total));
        let intervals: Vec<_> = ranges
            .iter()
            .map(|(a, b)| {
                (
                    (a * total - base).max(0.0),
                    (b * total - base).min(contour_length),
                )
            })
            .filter(|(a, b)| b > a)
            .collect();
        if intervals
            .iter()
            .any(|(a, b)| *a == 0.0 && *b == contour_length)
        {
            output.extend_from_slice(contour);
        } else if !intervals.is_empty() {
            trim_contour(contour, &intervals, contour_length, output);
        }
        offset += contour_length;
        start = end;
    }
}

fn trim_contour(
    elements: &[PathEl],
    intervals: &[(f64, f64)],
    length: f64,
    output: &mut Vec<PathEl>,
) {
    let segments: Vec<_> = segments(elements.iter().copied())
        .map(|seg| (seg, seg.arclen(ACCURACY)))
        .collect();
    let closed = matches!(elements.last(), Some(PathEl::ClosePath));
    let mut previous_end = None;
    for &(start, end) in intervals {
        let mut need_move = !(closed && previous_end == Some(length) && start == 0.0);
        let mut position = 0.0;
        for &(segment, segment_length) in &segments {
            if position >= end {
                break;
            }
            let next = position + segment_length;
            if segment_length > 0.0 && next > start && position < end {
                let t0 = if start <= position {
                    0.0
                } else {
                    segment
                        .inv_arclen(start - position, ACCURACY)
                        .clamp(0.0, 1.0)
                };
                let t1 = if end >= next {
                    1.0
                } else {
                    segment.inv_arclen(end - position, ACCURACY).clamp(0.0, 1.0)
                };
                if t1 > t0 {
                    let sub = segment.subsegment(t0..t1);
                    if need_move {
                        output.push(PathEl::MoveTo(sub.start()));
                        need_move = false;
                    }
                    output.push(sub.as_path_el());
                }
            }
            position = next;
        }
        previous_end = Some(end);
    }
}
