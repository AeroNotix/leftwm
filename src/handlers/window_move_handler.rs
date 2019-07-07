use super::*;

pub fn process(manager: &mut Manager, handle: &WindowHandle, offset_x: i32, offset_y: i32) -> bool {
    for w in &mut manager.windows {
        if &w.handle == handle {
            process_window(w, offset_x, offset_y);
            snap_to_workspaces(w, &manager.workspaces);
            return true;
        }
    }
    false
}

fn process_window(window: &mut Window, offset_x: i32, offset_y: i32) {
    window.set_floating(true);
    let mut offset = window
        .get_floating_offsets()
        .unwrap_or(XYHWBuilder::default().into());
    offset.set_x(offset_x);
    offset.set_y(offset_y);
    window.set_floating_offsets(Some(offset));
}

/*
fn process_window(window: &mut Window, offset_x: i32, offset_y: i32) {
    window.set_floating(true);
    if window.floating.is_none() {
        let mut floating: XYHW = XYHWBuilder::default().into();
        floating.set_w(window.width());
        floating.set_h(window.height());
        window.floating = Some(floating);
    }
    if window.start_loc.is_none() {
        let floating = window.floating.unwrap();
        window.start_loc = Some((floating.x(), floating.y()));
    }

    //they must have a value, it is safe to unwrap
    let floating = &mut window.floating.unwrap();
    let starting = &window.start_loc.unwrap();

    floating.set_x(starting.0 + offset_x);
    floating.set_y(starting.1 + offset_y);
    window.floating = Some(*floating);
}
 * */

//if the windows is really close to a workspace, snap to it
fn snap_to_workspaces(window: &mut Window, workspaces: &[Workspace]) -> bool {
    for workspace in workspaces {
        if snap_to_workspace(window, &workspace) {
            return true;
        }
    }
    false
}

fn snap_to_workspace(window: &mut Window, workspace: &Workspace) -> bool {
    if should_snap(window, workspace) {
        window.set_floating(false);
        window.tags = workspace.tags.clone();
        return true;
    }
    false
}

//to be snapable, the window must be inside the workspace AND the a side must be close to
//the workspaces edge
fn should_snap(window: &Window, workspace: &Workspace) -> bool {
    if window.must_float() {
        return false;
    }
    let loc = window.calculated_xyhw();
    //get window sides
    let win_left = loc.x();
    let win_right = win_left + window.width();
    let win_top = loc.y();
    let win_bottom = win_top + window.height();
    //check for conatins
    let center_x = loc.x() + (window.width() / 2);
    let center_y = loc.y() + (window.height() / 2);
    if !workspace.contains_point(center_x, center_y) {
        return false;
    }

    //check for close edge
    let dist = 10;
    let ws_left = workspace.x();
    let ws_right = workspace.x() + workspace.width();
    let ws_top = workspace.y();
    let ws_bottom = workspace.y() + workspace.height();
    if (win_top - ws_top).abs() < dist {
        return true;
    }
    if (win_bottom - ws_bottom).abs() < dist {
        return true;
    }
    if (win_left - ws_left).abs() < dist {
        return true;
    }
    if (win_right - ws_right).abs() < dist {
        return true;
    }
    false
}
