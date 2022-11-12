use schecker::schedule::{BusInfo, BusInfoWebsite, BusInfoDiff};
use std::collections::HashSet;

static TEST_REQUEST_TEXT: &str = r#"

<!DOCTYPE html>
<html>

<head>
    <title>Daily Reported Bus Route Issues</title>
    <base target="_top">
    <!--INCLUDE REQUIRED EXTERNAL JAVASCRIPT AND CSS LIBRARIES-->
    <script src=".\src\jquery-3.6.0.min.js"></script>
    <script src=".\src\jquery.dataTables.min.js"></script>
    <script src=".\src\dataTables.bootstrap4.min.js"></script>
    <link rel="stylesheet" type="text/css" href=".\src\bootstrap.css" />
    <link rel="stylesheet" type="text/css" href=".\src\dataTables.bootstrap4.min.css" />
    <style>
        .nobr 
        {
            white-space: nowrap;
            text-align: center;
        }
        .bold
        {
            font-weight: bold;
        }
        th
        {
            text-align: center;
        }
        #data-table span
        {
            display:none;
        }
    </style>

        <!--[if lt IE 12]>
            <script>
            window.open('https://support.microsoft.com/en-us/office/this-website-doesn-t-work-in-internet-explorer-8f5fc675-cd47-414c-9535-12821ddfc554?ui=en-us&rs=en-us&ad=us', '_blank');
            </script>
        <![endif]-->

    <script>
        var dataArray = [['94', '', 'JONES ES, SEVERNA PARK MS', 'PM', 'No Service', 'Sin Servicio']];
        $(document).ready(function () {
            $('#data-table').DataTable({
                destroy: true,
                bStateSave: true,
                data: dataArray,
                paging: false,
                order: [[0, "asc"]],
                columns: [
                    { "title": "Bus" },
                    { "title": "Sub Bus"},
                    { "title": "Schools" },
                    { "title": "Schedules" },
                    { "title": "Impact" },
                    { "title": "Impacto" }
                ]
            });
            setTimeout("window.open(self.location, '_self');", 60000 * 3);
        });
    </script>
</head>

</html>"#;

static TEST_REQUEST_TEXT_EMPTY: &str = r#"

<!DOCTYPE html>
<html>

<head>
    <title>Daily Reported Bus Route Issues</title>
    <base target="_top">
    <!--INCLUDE REQUIRED EXTERNAL JAVASCRIPT AND CSS LIBRARIES-->
    <script src=".\src\jquery-3.6.0.min.js"></script>
    <script src=".\src\jquery.dataTables.min.js"></script>
    <script src=".\src\dataTables.bootstrap4.min.js"></script>
    <link rel="stylesheet" type="text/css" href=".\src\bootstrap.css" />
    <link rel="stylesheet" type="text/css" href=".\src\dataTables.bootstrap4.min.css" />
    <style>
        .nobr 
        {
            white-space: nowrap;
            text-align: center;
        }
        .bold
        {
            font-weight: bold;
        }
        th
        {
            text-align: center;
        }
        #data-table span
        {
            display:none;
        }
    </style>

        <!--[if lt IE 12]>
            <script>
            window.open('https://support.microsoft.com/en-us/office/this-website-doesn-t-work-in-internet-explorer-8f5fc675-cd47-414c-9535-12821ddfc554?ui=en-us&rs=en-us&ad=us', '_blank');
            </script>
        <![endif]-->

    <script>
        var dataArray = [];
        $(document).ready(function () {
            $('#data-table').DataTable({
                destroy: true,
                bStateSave: true,
                data: dataArray,
                paging: false,
                order: [[0, "asc"]],
                columns: [
                    { "title": "Bus" },
                    { "title": "Sub Bus"},
                    { "title": "Schools" },
                    { "title": "Schedules" },
                    { "title": "Impact" },
                    { "title": "Impacto" }
                ]
            });
            setTimeout("window.open(self.location, '_self');", 60000 * 3);
        });
    </script>
</head>

</html>"#;

#[test]
fn schedule_parsing_with_text() {
    let website = BusInfoWebsite::Text(TEST_REQUEST_TEXT.to_string());
    let schedule = BusInfo::new(website);
    assert_eq!(
        schedule.current_schedule,
        vec![vec![
            "94",
            "",
            "PM",
            "JONES ES, SEVERNA PARK MS",
            "No Service",
            "Sin Servicio"
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()]
        .into_iter()
        .collect::<HashSet<Vec<String>>>()
    );
}

static TEST_REQUEST_TEXT_MULTIPLE: &str = r#"
<!DOCTYPE html>
<html>

<head>
    <title>Daily Reported Bus Route Issues</title>
    <base target="_top">
    <!--INCLUDE REQUIRED EXTERNAL JAVASCRIPT AND CSS LIBRARIES-->
    <script src=".\src\jquery-3.6.0.min.js"></script>
    <script src=".\src\jquery.dataTables.min.js"></script>
    <script src=".\src\dataTables.bootstrap4.min.js"></script>
    <link rel="stylesheet" type="text/css" href=".\src\bootstrap.css" />
    <link rel="stylesheet" type="text/css" href=".\src\dataTables.bootstrap4.min.css" />
    <style>
        .nobr 
        {
            white-space: nowrap;
            text-align: center;
        }
        .bold
        {
            font-weight: bold;
        }
        th
        {
            text-align: center;
        }
        #data-table span
        {
            display:none;
        }
    </style>

        <!--[if lt IE 12]>
            <script>
            window.open('https://support.microsoft.com/en-us/office/this-website-doesn-t-work-in-internet-explorer-8f5fc675-cd47-414c-9535-12821ddfc554?ui=en-us&rs=en-us&ad=us', '_blank');
            </script>
        <![endif]-->
r
    <script>
        var dataArray = [['1', '', 'DEALE ES, SOUTHERN HS, SOUTHERN MS', 'AM & PM', 'No Service', 'Sin Servicio'], ['4', '', 'OLD MILL MIDDLE SCHOOL SOUTH', 'PM', 'No Service', 'Sin Servicio'], ['25', '491', 'OLD MILL MIDDLE SCHOOL SOUTH', 'AM', 'No Impact', 'Sin Impacto'], ['25', '', 'OLD MILL MIDDLE SCHOOL SOUTH', 'PM', 'No Service', 'Sin Servicio'], ['26', '', 'SHADY SIDE ES, SOUTHERN MS', 'AM & PM', 'No Service', 'Sin Servicio'], ['32', '', 'SOUTHERN HS', 'AM & PM', 'No Service', 'Sin Servicio'], ['42', '', 'GERMANTOWN ES', 'AM', 'No Service', 'Sin Servicio'], ['42', '467', 'GERMANTOWN ES', 'PM', 'No Impact', 'Sin Impacto'], ['44', '', 'NORTH COUNTY HS', 'AM & PM', 'No Service', 'Sin Servicio'], ['52', '132', 'CHESAPEAKE HS', 'AM & PM', 'No Impact', 'Sin Impacto'], ['52', '294', 'CHESAPEAKE BAY MS', 'AM & PM', 'No Impact', 'Sin Impacto'], ['82', '', 'ANNAPOLIS HS', 'AM & PM', 'No Service', 'Sin Servicio'], ['113', '466', 'CHESAPEAKE HS', 'AM & PM', 'Less than 20 minutes', 'Menos de 20 minutos'], ['113', '290', 'CHESAPEAKE BAY MS', 'PM', 'No Impact', 'Sin Impacto'], ['113', '294', 'BODKIN ES', 'AM & PM', 'Less than 20 minutes', 'Menos de 20 minutos'], ['132', '290', 'NORTHEAST HS', 'AM & PM', 'No Impact', 'Sin Impacto'], ['149', '216', 'BATES MS', 'AM & PM', 'No Impact', 'Sin Impacto'], ['159', '149', 'BATES MS', 'AM & PM', 'No Impact', 'Sin Impacto'], ['177', '', 'CROFTON HS', 'AM & PM', 'No Service', 'Sin Servicio'], ['177', '', 'PINEY ORCHARD ES', 'AM & PM', 'No Service', 'Sin Servicio'], ['189', '', 'ANNAPOLIS HS, BATES MS, ROLLING KNOLLS ES', 'AM & PM', 'No Service', 'Sin Servicio'], ['201', '', 'BROADNECK HS', 'AM & PM', 'No Service', 'Sin Servicio'], ['225', '', 'BROADNECK ES, BROADNECK HS, SEVERN RIVER MS', 'AM & PM', 'No Service', 'Sin Servicio'], ['230', '', 'MEADE HS, MEADE MS, SEVERN ES', 'AM & PM', 'No Service', 'Sin Servicio'], ['238', '', 'BROADNECK HS', 'AM', 'No Service', 'Sin Servicio'], ['238', '187', 'BROADNECK HS', 'PM', 'No Impact', 'Sin Impacto'], ['238', '', 'BROADNECK ES, MAGOTHY RIVER MS', 'AM & PM', 'No Service', 'Sin Servicio'], ['242', '703', 'NORTHEAST MS', 'AM & PM', 'No Impact', 'Sin Impacto'], ['243', '', 'CENTRAL ES', 'AM & PM', 'No Service', 'Sin Servicio'], ['248', '', 'SOUTHERN MS, TRACEY\'S ES', 'AM & PM', 'No Service', 'Sin Servicio'], ['274', '', 'ANNAPOLIS HS', 'AM & PM', 'No Service', 'Sin Servicio'], ['286', '', 'ARNOLD ES, SEVERN RIVER MS', 'AM & PM', 'No Service', 'Sin Servicio'], ['291', '', 'NORTH COUNTY HS', 'AM & PM', 'No Service', 'Sin Servicio'], ['297', '', 'SHADY SIDE ES', 'AM', 'No Service', 'Sin Servicio'], ['297', '', 'SOUTHERN MS', 'AM & PM', 'No Service', 'Sin Servicio'], ['297', '37', 'SHADY SIDE ES', 'PM', 'No Impact', 'Sin Impacto'], ['303', '', 'CAPE ST. CLAIRE ES, SEVERN RIVER MS', 'AM & PM', 'No Service', 'Sin Servicio'], ['303', '', 'BROADNECK HS', 'PM', 'No Service', 'Sin Servicio'], ['303', '231', 'BROADNECK HS', 'AM', 'No Impact', 'Sin Impacto'], ['333', '', 'JESSUP ES, MEADE HS, MEADE MS', 'AM & PM', 'No Service', 'Sin Servicio'], ['337', '', 'BROADNECK ES, MAGOTHY RIVER MS', 'AM', 'No Service', 'Sin Servicio'], ['343', '', 'CENTRAL MS, MAYO ES, SOUTH RIVER HS', 'AM & PM', 'No Service', 'Sin Servicio'], ['376', '', 'ANNAPOLIS HS, BATES MS, MILLS-PAROLE ES', 'PM', 'No Service', 'Sin Servicio'], ['387', '611', 'SEVERN RIVER MS', 'PM', 'No Impact', 'Sin Impacto'], ['387', '', 'SEVERN RIVER MS', 'PM', 'No Service', 'Sin Servicio'], ['387', '611', 'WINDSOR FARM ES', 'PM', 'No Impact', 'Sin Impacto'], ['387', '', 'WINDSOR FARM ES', 'AM', 'No Service', 'Sin Servicio'], ['387', '', 'BROADNECK HS', 'AM & PM', 'No Service', 'Sin Servicio'], ['393', '', 'WINDSOR FARM ES', 'AM & PM', 'No Service', 'Sin Servicio'], ['412', '', 'NORTH COUNTY HS', 'AM & PM', 'No Service', 'Sin Servicio'], ['413', '', 'NORTH COUNTY HS', 'AM', 'No Service', 'Sin Servicio'], ['426', '337', 'BROADNECK HS', 'PM', 'No Impact', 'Sin Impacto'], ['426', '', 'ARNOLD ES, MAGOTHY RIVER MS', 'AM & PM', 'No Service', 'Sin Servicio'], ['426', '', 'BROADNECK HS', 'AM', 'No Service', 'Sin Servicio'], ['427', '', 'SOUTH RIVER HS', 'AM & PM', 'No Service', 'Sin Servicio'], ['428', '', 'CENTRAL ES, SOUTH RIVER HS', 'AM', 'No Service', 'Sin Servicio'], ['440', '394', 'CORKRAN MS', 'AM & PM', 'Less than 20 minutes', 'Menos de 20 minutos'], ['440', '555', 'GLEN BURNIE HS', 'AM & PM', 'No Impact', 'Sin Impacto'], ['442', '118', 'GLEN BURNIE HS', 'PM', 'No Impact', 'Sin Impacto'], ['442', '152', 'GLEN BURNIE HS', 'AM', 'No Impact', 'Sin Impacto'], ['442', '147', 'FREETOWN ES', 'AM & PM', 'No Impact', 'Sin Impacto'], ['461', '461', 'CENTRAL MS, DAVIDSONVILLE ES, SOUTH RIVER HS', 'AM & PM', 'No Impact', 'Sin Impacto'], ['641', '', 'SOUTHERN MS, TRACEY\'S ES', 'AM & PM', 'No Service', 'Sin Servicio'], ['689', '', 'SHEPPARD PRATT-LANHAM', 'AM & PM', 'No Service', 'Sin Servicio'], ['808', '', 'BENFIELD ES', 'AM & PM', 'No Impact', 'Sin Impacto'], ['816', '', 'WAUGH CHAPEL ES', 'AM & PM', 'No Impact', 'Sin Impacto'], ['816', '', 'RUTH PARKER EASON SCHOOL', 'AM & PM', 'No Impact', 'Sin Impacto'], ['860', '', 'POINT PLEASANT ES', 'AM & PM', 'No Impact', 'Sin Impacto']];
        $(document).ready(function () {
            $('#data-table').DataTable({
                destroy: true,
                bStateSave: true,
                data: dataArray,
                paging: false,
                order: [[0, "asc"]],
                columns: [
                    { "title": "Bus" },
                    { "title": "Sub Bus"},
                    { "title": "Schools" },
                    { "title": "Schedules" },
                    { "title": "Impact" },
                    { "title": "Impacto" }
                ]
            });
            setTimeout("window.open(self.location, '_self');", 60000 * 3);
        });
    </script>
</head>"#;

#[test]
fn schedule_parsing_empty() {
    let website = BusInfoWebsite::Text(TEST_REQUEST_TEXT_EMPTY.to_string());
    let schedule = BusInfo::new(website);
    assert_eq!(schedule.current_schedule, Vec::new().into_iter().collect::<HashSet<_>>());
}

#[test]
fn schedule_diff_from_empty() {
    let empty_website = BusInfoWebsite::Text(TEST_REQUEST_TEXT_EMPTY.to_string());
    let empty_schedule = BusInfo::new(empty_website);
    let norm_website = BusInfoWebsite::Text(TEST_REQUEST_TEXT.to_string());
    let updated_schedule = empty_schedule.update(norm_website);
    assert_eq!(updated_schedule.diff(), BusInfoDiff { new: Some(updated_schedule.current_schedule), updated: None, now_running: None });
}

#[test]
fn schedule_diff_to_empty() {
    let norm_website = BusInfoWebsite::Text(TEST_REQUEST_TEXT.to_string());
    let empty_website = BusInfoWebsite::Text(TEST_REQUEST_TEXT_EMPTY.to_string());
    let norm_schedule = BusInfo::new(norm_website);
    let updated_schedule = norm_schedule.update(empty_website);
    assert_eq!(updated_schedule.diff(), BusInfoDiff { now_running: Some(updated_schedule.previous_schedule.unwrap()), updated: None, new: None });
}

#[test]
fn schedule_diff_mixed() {
    let norm_website = BusInfoWebsite::Text(TEST_REQUEST_TEXT.to_string());
    let empty_website = BusInfoWebsite::Text(TEST_REQUEST_TEXT_MULTIPLE.to_string());
    let norm_schedule = BusInfo::new(norm_website);
    let updated_schedule = norm_schedule.update(empty_website);
    assert_eq!(updated_schedule.diff(), BusInfoDiff { now_running: Some(updated_schedule.previous_schedule.unwrap()), updated: None, new: Some(updated_schedule.current_schedule) });

}
