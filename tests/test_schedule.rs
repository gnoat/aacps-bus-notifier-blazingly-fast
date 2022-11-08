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

<body>
    <div class="container">
        <br />
        <img src=".\src\aa-logo.png" />
        <br />
        <!-- The jQuery DataTable does not always work well with IE, the below message will display for users who use IE as their browser -->
        <!--[if lt IE 12]>
            <br>
            <p>This page may not display correctly in Internet Explorer, please consider using a more modern browser to improve your experience.</p>
            <br>
        <![endif]-->
        <br />
        AACPS bus contractors have reported the following issues with bus runs scheduled for <span class="bold"><span id="Label_Message">11/8/2022</span></span>. 
        Should you have further questions, please contact the Transportation Division at 410-923-7890. 
        Buses appear in the chart below as soon as they are entered by contractors. 
        The columns below can each be sorted by clicking the up/down arrows next to the column title. 
        Entering a bus number into the search bar will filter out all entries other than those matching the search criteria.
        (This page should refresh every 3 minutes).
        <br /><br />
        Los contratistas de autobuses de AACPS han informado de los siguientes problemas con las carreras de autobuses programadas para el <span class="bold"><span id="Label_SpanishMessage">11/8/2022</span></span>. 
        Si tiene más preguntas, comuníquese con la División de Transporte al 410-923-7890. 
        Los autobuses aparecen en la tabla a continuación tan pronto como son ingresados por los contratistas. 
        Las columnas a continuación se pueden ordenar haciendo clic en las flechas hacia arriba / hacia abajo junto al título de la columna. 
        Al introducir un número de bus en la barra de búsqueda, se filtrarán todas las entradas que no coincidan con los criterios de búsqueda. 
        (Esta página debe actualizarse cada 3 minutos).
        
        <div id="Panel_BusesFound">
	
        <br />
        <br />
        <div class="row">
            <table id="data-table" class="table table-striped table-sm table-hover table-bordered">
                <!-- TABLE DATA IS ADDED BY THE showData() JAVASCRIPT FUNCTION ABOVE -->
            </table>
        </div>
        
</div>
    </div>
</body>

</html>"#;

#[test]
fn schedule_parsing() {
    let website = BusInfoWebsite::Text(TEST_REQUEST_TEXT.to_string());
    let schedule = BusInfo::new(website);
    assert_eq!(
        schedule.current_schedule,
        vec![vec![
            "94".to_string(),
            "".to_string(),
            "PM".to_string(),
            "JONES ES, SEVERNA PARK MS".to_string(),
            "No Service".to_string(),
            "Sin Servicio".to_string()
        ]]
        .into_iter()
        .collect::<HashSet<Vec<String>>>()
    );
}
