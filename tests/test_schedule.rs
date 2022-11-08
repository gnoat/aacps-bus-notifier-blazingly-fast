use busnote::schedule::{BusInfo, BusInfoWebsite};
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
        var dataArray = [[]];
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

#[test]
fn schedule_parsing_empty() {
    let website = BusInfoWebsite::Text(TEST_REQUEST_TEXT_EMPTY.to_string());
    let schedule = BusInfo::new(website);
    assert_eq!(schedule.current_schedule, Vec::new().into_iter().collect::<HashSet<_>>());
}
