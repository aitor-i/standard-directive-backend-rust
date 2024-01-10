use warp::Filter;

use crate::controllers::{
    get_calendar_by_date_controller::get_calendar_by_date_controller,
    get_calendars_controller::get_calendars_controller,
    post_save_event_controller::post_save_event_controller,
};

pub fn calendar_router() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path("calendar").and(
        get_calendar_by_date_controller()
            .or(get_calendars_controller())
            .or(post_save_event_controller()),
    )
}
