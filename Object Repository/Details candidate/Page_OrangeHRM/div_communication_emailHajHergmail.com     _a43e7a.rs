<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_communication_emailHajHergmail.com     _a43e7a</name>
   <tag></tag>
   <elementGuidId>3a58f5e3-efd1-440b-89aa-4ffffdf04465</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='candidate-profile-div']</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>#candidate-profile-div</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>col l12 pageholder</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>candidate-profile-div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    


    
    
        
            
                
                
                    
                        
                                    
                
                
                                                                
                
                    
                        communication_email HajHer@gmail.com
                    
                    
                        communication_phone 
                    
                

            
            
                Details
                Resume
                Notes
            
        
    
            





    


    isInterviewAssistantOnInLicense = true;
    var getVacancyEventLastCommentURL = '/recruitment/getVacancyEventLastComment';
    var lang_Collapse = 'Collapse';
    var lang_Expand = 'Expand';

    
        



    
        Details
        form_download

    


    

        
                                    
                
                    
                        UI/UX Engineer
                             
                            
                        
                --Select--
        
                        
                Credit Analyst
        
                        
                Customer Success Executive
        
                        
                DevOps Engineer
        
                        
                Financial Analyst
        
                        
                Production Manager
        
                        
                Sales Coordinator
        
                        
                Sales Manager
        
                        
                Software Engineer
        
                        
                Software QA Engineer
        
                        
                Technical Support Engineer
        
                        
                UI/UX Engineer
        $(&quot;.dropdown-employees&quot;).dropdown({
            inDuration: 300,
            outDuration: 225,
            constrain_width: false, 
            hover: false, 
            gutter: 200, 
            belowOrigin: false
          });                        Vacancy                    
                
                
                    Software Engineer
                    Job Title
                

                
                    
                
            
            
                

                    
                Personal Details
                    

                    
                        
                                                        First Name *                        
                        
                                                        Middle Name                        
                        
                                                        Last Name *                        
                    
                    
                        
                                                        E-mail *                        
                        
                                                        Contact Number                        

                        
                            SaturdayMay292021JanuaryFebruaryMarchAprilMayJuneJulyAugustSeptemberOctoberNovemberDecember20162017201820192020202120222023202420252026  SMTWTFS2526272829301234567891011121314151617181920212223242526272829303112345TodayClearClose            
              var isIE = navigator.userAgent.indexOf(&quot;MSIE &quot;) > -1 || navigator.userAgent.indexOf(&quot;Trident/&quot;) > -1 || navigator.userAgent.indexOf(&quot;Edge/&quot;) > -1;
              var pickerOpened = false;
              
              $('.datepicker').pickadate({  
               format: getDatePickerCompatibleDateFormat(&quot;D, d M Y&quot;),
                onSet: function (ele) {
                    if(ele.select){
                           this.close();
                    }
                 },
                today: &quot;Today&quot;,
                clear: &quot;Clear&quot;,
                close: &quot;Close&quot;,

                labelMonthNext: &quot;Next month&quot;,
                labelMonthPrev: &quot;Previous month&quot;,
                labelMonthSelect: &quot;Select a month&quot;,

                selectMonths: true,
                selectYears: '10',
                onClose: function() {
                    var element = this.$node;
                    element.removeClass('active_datepicker');
                    $('.datepicker').blur();
                    $('.picker').blur();
                    pickerOpened = false;
                },
                onOpen: function() {
                    if (!pickerOpened &amp;&amp; isIE) {
                        this.close();
                        return;
                    }
                    var element = this.$node;
                    element.addClass('active_datepicker');
                    
                    fixDropDown(this.$root.find('.picker__select--year'));
                    fixDropDown(this.$root.find('.picker__select--month'));
                    
                    this.$root.mouseup(function (e) {
                            var containers = $(this).find(&quot;.select-dropdown&quot;);

                            containers.each(function () {
                                if (!$(this).is(e.target) &amp;&amp; $(this).has(e.target).length === 0) 
                                {
                                    $(this).siblings('input.select-dropdown').trigger('close');
                                }
                            });

                        });
                },
                onRender: function () {
                var pickerInstanceElement = this;
                this.$node.bind('click', function (e) {
                    e.stopPropagation();
                    pickerInstanceElement.open();
                    pickerOpened = true;
                });
                    var offDays={&quot;1578117600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 01 04&quot;,&quot;original-date&quot;:&quot;2020-01-04&quot;},&quot;1578204000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 01 05&quot;,&quot;original-date&quot;:&quot;2020-01-05&quot;},&quot;1578722400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 01 11&quot;,&quot;original-date&quot;:&quot;2020-01-11&quot;},&quot;1578808800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 01 12&quot;,&quot;original-date&quot;:&quot;2020-01-12&quot;},&quot;1579327200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 01 18&quot;,&quot;original-date&quot;:&quot;2020-01-18&quot;},&quot;1579413600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 01 19&quot;,&quot;original-date&quot;:&quot;2020-01-19&quot;},&quot;1579932000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 01 25&quot;,&quot;original-date&quot;:&quot;2020-01-25&quot;},&quot;1580018400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 01 26&quot;,&quot;original-date&quot;:&quot;2020-01-26&quot;},&quot;1580536800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 02 01&quot;,&quot;original-date&quot;:&quot;2020-02-01&quot;},&quot;1580623200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 02 02&quot;,&quot;original-date&quot;:&quot;2020-02-02&quot;},&quot;1581141600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 02 08&quot;,&quot;original-date&quot;:&quot;2020-02-08&quot;},&quot;1581228000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 02 09&quot;,&quot;original-date&quot;:&quot;2020-02-09&quot;},&quot;1581746400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 02 15&quot;,&quot;original-date&quot;:&quot;2020-02-15&quot;},&quot;1581832800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 02 16&quot;,&quot;original-date&quot;:&quot;2020-02-16&quot;},&quot;1582351200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 02 22&quot;,&quot;original-date&quot;:&quot;2020-02-22&quot;},&quot;1582437600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 02 23&quot;,&quot;original-date&quot;:&quot;2020-02-23&quot;},&quot;1582956000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 02 29&quot;,&quot;original-date&quot;:&quot;2020-02-29&quot;},&quot;1583042400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 03 01&quot;,&quot;original-date&quot;:&quot;2020-03-01&quot;},&quot;1583560800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 03 07&quot;,&quot;original-date&quot;:&quot;2020-03-07&quot;},&quot;1583647200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 03 08&quot;,&quot;original-date&quot;:&quot;2020-03-08&quot;},&quot;1584162000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 03 14&quot;,&quot;original-date&quot;:&quot;2020-03-14&quot;},&quot;1584248400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 03 15&quot;,&quot;original-date&quot;:&quot;2020-03-15&quot;},&quot;1584766800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 03 21&quot;,&quot;original-date&quot;:&quot;2020-03-21&quot;},&quot;1584853200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 03 22&quot;,&quot;original-date&quot;:&quot;2020-03-22&quot;},&quot;1585371600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 03 28&quot;,&quot;original-date&quot;:&quot;2020-03-28&quot;},&quot;1585458000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 03 29&quot;,&quot;original-date&quot;:&quot;2020-03-29&quot;},&quot;1585976400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 04 04&quot;,&quot;original-date&quot;:&quot;2020-04-04&quot;},&quot;1586062800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 04 05&quot;,&quot;original-date&quot;:&quot;2020-04-05&quot;},&quot;1586581200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 04 11&quot;,&quot;original-date&quot;:&quot;2020-04-11&quot;},&quot;1586667600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 04 12&quot;,&quot;original-date&quot;:&quot;2020-04-12&quot;},&quot;1587186000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 04 18&quot;,&quot;original-date&quot;:&quot;2020-04-18&quot;},&quot;1587272400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 04 19&quot;,&quot;original-date&quot;:&quot;2020-04-19&quot;},&quot;1587790800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 04 25&quot;,&quot;original-date&quot;:&quot;2020-04-25&quot;},&quot;1587877200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 04 26&quot;,&quot;original-date&quot;:&quot;2020-04-26&quot;},&quot;1588395600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 05 02&quot;,&quot;original-date&quot;:&quot;2020-05-02&quot;},&quot;1588482000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 05 03&quot;,&quot;original-date&quot;:&quot;2020-05-03&quot;},&quot;1589000400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 05 09&quot;,&quot;original-date&quot;:&quot;2020-05-09&quot;},&quot;1589086800&quot;:{&quot;type&quot;:&quot;holiday&quot;,&quot;duration&quot;:&quot;0&quot;,&quot;date&quot;:&quot;2020 05 10&quot;,&quot;original-date&quot;:&quot;2020-05-10&quot;},&quot;1589605200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 05 16&quot;,&quot;original-date&quot;:&quot;2020-05-16&quot;},&quot;1589691600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 05 17&quot;,&quot;original-date&quot;:&quot;2020-05-17&quot;},&quot;1590210000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 05 23&quot;,&quot;original-date&quot;:&quot;2020-05-23&quot;},&quot;1590296400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 05 24&quot;,&quot;original-date&quot;:&quot;2020-05-24&quot;},&quot;1590814800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 05 30&quot;,&quot;original-date&quot;:&quot;2020-05-30&quot;},&quot;1590901200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 05 31&quot;,&quot;original-date&quot;:&quot;2020-05-31&quot;},&quot;1591419600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 06 06&quot;,&quot;original-date&quot;:&quot;2020-06-06&quot;},&quot;1591506000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 06 07&quot;,&quot;original-date&quot;:&quot;2020-06-07&quot;},&quot;1592024400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 06 13&quot;,&quot;original-date&quot;:&quot;2020-06-13&quot;},&quot;1592110800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 06 14&quot;,&quot;original-date&quot;:&quot;2020-06-14&quot;},&quot;1592629200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 06 20&quot;,&quot;original-date&quot;:&quot;2020-06-20&quot;},&quot;1592715600&quot;:{&quot;type&quot;:&quot;holiday&quot;,&quot;duration&quot;:&quot;0&quot;,&quot;date&quot;:&quot;2020 06 21&quot;,&quot;original-date&quot;:&quot;2020-06-21&quot;},&quot;1593234000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 06 27&quot;,&quot;original-date&quot;:&quot;2020-06-27&quot;},&quot;1593320400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 06 28&quot;,&quot;original-date&quot;:&quot;2020-06-28&quot;},&quot;1593838800&quot;:{&quot;type&quot;:&quot;holiday&quot;,&quot;duration&quot;:&quot;0&quot;,&quot;date&quot;:&quot;2020 07 04&quot;,&quot;original-date&quot;:&quot;2020-07-04&quot;},&quot;1593925200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 07 05&quot;,&quot;original-date&quot;:&quot;2020-07-05&quot;},&quot;1594443600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 07 11&quot;,&quot;original-date&quot;:&quot;2020-07-11&quot;},&quot;1594530000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 07 12&quot;,&quot;original-date&quot;:&quot;2020-07-12&quot;},&quot;1595048400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 07 18&quot;,&quot;original-date&quot;:&quot;2020-07-18&quot;},&quot;1595134800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 07 19&quot;,&quot;original-date&quot;:&quot;2020-07-19&quot;},&quot;1595653200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 07 25&quot;,&quot;original-date&quot;:&quot;2020-07-25&quot;},&quot;1595739600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 07 26&quot;,&quot;original-date&quot;:&quot;2020-07-26&quot;},&quot;1596258000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 08 01&quot;,&quot;original-date&quot;:&quot;2020-08-01&quot;},&quot;1596344400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 08 02&quot;,&quot;original-date&quot;:&quot;2020-08-02&quot;},&quot;1596862800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 08 08&quot;,&quot;original-date&quot;:&quot;2020-08-08&quot;},&quot;1596949200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 08 09&quot;,&quot;original-date&quot;:&quot;2020-08-09&quot;},&quot;1597467600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 08 15&quot;,&quot;original-date&quot;:&quot;2020-08-15&quot;},&quot;1597554000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 08 16&quot;,&quot;original-date&quot;:&quot;2020-08-16&quot;},&quot;1598072400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 08 22&quot;,&quot;original-date&quot;:&quot;2020-08-22&quot;},&quot;1598158800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 08 23&quot;,&quot;original-date&quot;:&quot;2020-08-23&quot;},&quot;1598677200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 08 29&quot;,&quot;original-date&quot;:&quot;2020-08-29&quot;},&quot;1598763600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 08 30&quot;,&quot;original-date&quot;:&quot;2020-08-30&quot;},&quot;1599282000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 09 05&quot;,&quot;original-date&quot;:&quot;2020-09-05&quot;},&quot;1599368400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 09 06&quot;,&quot;original-date&quot;:&quot;2020-09-06&quot;},&quot;1599886800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 09 12&quot;,&quot;original-date&quot;:&quot;2020-09-12&quot;},&quot;1599973200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 09 13&quot;,&quot;original-date&quot;:&quot;2020-09-13&quot;},&quot;1600491600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 09 19&quot;,&quot;original-date&quot;:&quot;2020-09-19&quot;},&quot;1600578000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 09 20&quot;,&quot;original-date&quot;:&quot;2020-09-20&quot;},&quot;1601096400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 09 26&quot;,&quot;original-date&quot;:&quot;2020-09-26&quot;},&quot;1601182800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 09 27&quot;,&quot;original-date&quot;:&quot;2020-09-27&quot;},&quot;1601701200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 10 03&quot;,&quot;original-date&quot;:&quot;2020-10-03&quot;},&quot;1601787600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 10 04&quot;,&quot;original-date&quot;:&quot;2020-10-04&quot;},&quot;1602306000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 10 10&quot;,&quot;original-date&quot;:&quot;2020-10-10&quot;},&quot;1602392400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 10 11&quot;,&quot;original-date&quot;:&quot;2020-10-11&quot;},&quot;1602910800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 10 17&quot;,&quot;original-date&quot;:&quot;2020-10-17&quot;},&quot;1602997200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 10 18&quot;,&quot;original-date&quot;:&quot;2020-10-18&quot;},&quot;1603515600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 10 24&quot;,&quot;original-date&quot;:&quot;2020-10-24&quot;},&quot;1603602000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 10 25&quot;,&quot;original-date&quot;:&quot;2020-10-25&quot;},&quot;1604120400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 10 31&quot;,&quot;original-date&quot;:&quot;2020-10-31&quot;},&quot;1604206800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 11 01&quot;,&quot;original-date&quot;:&quot;2020-11-01&quot;},&quot;1604728800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 11 07&quot;,&quot;original-date&quot;:&quot;2020-11-07&quot;},&quot;1604815200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 11 08&quot;,&quot;original-date&quot;:&quot;2020-11-08&quot;},&quot;1605333600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 11 14&quot;,&quot;original-date&quot;:&quot;2020-11-14&quot;},&quot;1605420000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 11 15&quot;,&quot;original-date&quot;:&quot;2020-11-15&quot;},&quot;1605938400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 11 21&quot;,&quot;original-date&quot;:&quot;2020-11-21&quot;},&quot;1606024800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 11 22&quot;,&quot;original-date&quot;:&quot;2020-11-22&quot;},&quot;1606543200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 11 28&quot;,&quot;original-date&quot;:&quot;2020-11-28&quot;},&quot;1606629600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 11 29&quot;,&quot;original-date&quot;:&quot;2020-11-29&quot;},&quot;1607148000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 12 05&quot;,&quot;original-date&quot;:&quot;2020-12-05&quot;},&quot;1607234400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 12 06&quot;,&quot;original-date&quot;:&quot;2020-12-06&quot;},&quot;1607752800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 12 12&quot;,&quot;original-date&quot;:&quot;2020-12-12&quot;},&quot;1607839200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 12 13&quot;,&quot;original-date&quot;:&quot;2020-12-13&quot;},&quot;1608357600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 12 19&quot;,&quot;original-date&quot;:&quot;2020-12-19&quot;},&quot;1608444000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 12 20&quot;,&quot;original-date&quot;:&quot;2020-12-20&quot;},&quot;1608962400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 12 26&quot;,&quot;original-date&quot;:&quot;2020-12-26&quot;},&quot;1609048800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2020 12 27&quot;,&quot;original-date&quot;:&quot;2020-12-27&quot;},&quot;1609567200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 01 02&quot;,&quot;original-date&quot;:&quot;2021-01-02&quot;},&quot;1609653600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 01 03&quot;,&quot;original-date&quot;:&quot;2021-01-03&quot;},&quot;1610172000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 01 09&quot;,&quot;original-date&quot;:&quot;2021-01-09&quot;},&quot;1610258400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 01 10&quot;,&quot;original-date&quot;:&quot;2021-01-10&quot;},&quot;1610776800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 01 16&quot;,&quot;original-date&quot;:&quot;2021-01-16&quot;},&quot;1610863200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 01 17&quot;,&quot;original-date&quot;:&quot;2021-01-17&quot;},&quot;1611381600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 01 23&quot;,&quot;original-date&quot;:&quot;2021-01-23&quot;},&quot;1611468000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 01 24&quot;,&quot;original-date&quot;:&quot;2021-01-24&quot;},&quot;1611986400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 01 30&quot;,&quot;original-date&quot;:&quot;2021-01-30&quot;},&quot;1612072800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 01 31&quot;,&quot;original-date&quot;:&quot;2021-01-31&quot;},&quot;1612591200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 02 06&quot;,&quot;original-date&quot;:&quot;2021-02-06&quot;},&quot;1612677600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 02 07&quot;,&quot;original-date&quot;:&quot;2021-02-07&quot;},&quot;1613196000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 02 13&quot;,&quot;original-date&quot;:&quot;2021-02-13&quot;},&quot;1613282400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 02 14&quot;,&quot;original-date&quot;:&quot;2021-02-14&quot;},&quot;1613800800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 02 20&quot;,&quot;original-date&quot;:&quot;2021-02-20&quot;},&quot;1613887200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 02 21&quot;,&quot;original-date&quot;:&quot;2021-02-21&quot;},&quot;1614405600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 02 27&quot;,&quot;original-date&quot;:&quot;2021-02-27&quot;},&quot;1614492000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 02 28&quot;,&quot;original-date&quot;:&quot;2021-02-28&quot;},&quot;1615010400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 03 06&quot;,&quot;original-date&quot;:&quot;2021-03-06&quot;},&quot;1615096800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 03 07&quot;,&quot;original-date&quot;:&quot;2021-03-07&quot;},&quot;1615615200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 03 13&quot;,&quot;original-date&quot;:&quot;2021-03-13&quot;},&quot;1615701600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 03 14&quot;,&quot;original-date&quot;:&quot;2021-03-14&quot;},&quot;1616216400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 03 20&quot;,&quot;original-date&quot;:&quot;2021-03-20&quot;},&quot;1616302800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 03 21&quot;,&quot;original-date&quot;:&quot;2021-03-21&quot;},&quot;1616821200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 03 27&quot;,&quot;original-date&quot;:&quot;2021-03-27&quot;},&quot;1616907600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 03 28&quot;,&quot;original-date&quot;:&quot;2021-03-28&quot;},&quot;1617426000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 04 03&quot;,&quot;original-date&quot;:&quot;2021-04-03&quot;},&quot;1617512400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 04 04&quot;,&quot;original-date&quot;:&quot;2021-04-04&quot;},&quot;1618030800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 04 10&quot;,&quot;original-date&quot;:&quot;2021-04-10&quot;},&quot;1618117200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 04 11&quot;,&quot;original-date&quot;:&quot;2021-04-11&quot;},&quot;1618635600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 04 17&quot;,&quot;original-date&quot;:&quot;2021-04-17&quot;},&quot;1618722000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 04 18&quot;,&quot;original-date&quot;:&quot;2021-04-18&quot;},&quot;1619240400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 04 24&quot;,&quot;original-date&quot;:&quot;2021-04-24&quot;},&quot;1619326800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 04 25&quot;,&quot;original-date&quot;:&quot;2021-04-25&quot;},&quot;1619845200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 05 01&quot;,&quot;original-date&quot;:&quot;2021-05-01&quot;},&quot;1619931600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 05 02&quot;,&quot;original-date&quot;:&quot;2021-05-02&quot;},&quot;1620450000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 05 08&quot;,&quot;original-date&quot;:&quot;2021-05-08&quot;},&quot;1620536400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 05 09&quot;,&quot;original-date&quot;:&quot;2021-05-09&quot;},&quot;1621054800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 05 15&quot;,&quot;original-date&quot;:&quot;2021-05-15&quot;},&quot;1621141200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 05 16&quot;,&quot;original-date&quot;:&quot;2021-05-16&quot;},&quot;1621659600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 05 22&quot;,&quot;original-date&quot;:&quot;2021-05-22&quot;},&quot;1621746000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 05 23&quot;,&quot;original-date&quot;:&quot;2021-05-23&quot;},&quot;1622264400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 05 29&quot;,&quot;original-date&quot;:&quot;2021-05-29&quot;},&quot;1622350800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 05 30&quot;,&quot;original-date&quot;:&quot;2021-05-30&quot;},&quot;1622869200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 06 05&quot;,&quot;original-date&quot;:&quot;2021-06-05&quot;},&quot;1622955600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 06 06&quot;,&quot;original-date&quot;:&quot;2021-06-06&quot;},&quot;1623474000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 06 12&quot;,&quot;original-date&quot;:&quot;2021-06-12&quot;},&quot;1623560400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 06 13&quot;,&quot;original-date&quot;:&quot;2021-06-13&quot;},&quot;1624078800&quot;:{&quot;type&quot;:&quot;holiday&quot;,&quot;duration&quot;:&quot;0&quot;,&quot;date&quot;:&quot;2021 06 19&quot;,&quot;original-date&quot;:&quot;2021-06-19&quot;},&quot;1624165200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 06 20&quot;,&quot;original-date&quot;:&quot;2021-06-20&quot;},&quot;1624683600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 06 26&quot;,&quot;original-date&quot;:&quot;2021-06-26&quot;},&quot;1624770000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 06 27&quot;,&quot;original-date&quot;:&quot;2021-06-27&quot;},&quot;1625288400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 07 03&quot;,&quot;original-date&quot;:&quot;2021-07-03&quot;},&quot;1625374800&quot;:{&quot;type&quot;:&quot;holiday&quot;,&quot;duration&quot;:&quot;0&quot;,&quot;date&quot;:&quot;2021 07 04&quot;,&quot;original-date&quot;:&quot;2021-07-04&quot;},&quot;1625893200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 07 10&quot;,&quot;original-date&quot;:&quot;2021-07-10&quot;},&quot;1625979600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 07 11&quot;,&quot;original-date&quot;:&quot;2021-07-11&quot;},&quot;1626498000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 07 17&quot;,&quot;original-date&quot;:&quot;2021-07-17&quot;},&quot;1626584400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 07 18&quot;,&quot;original-date&quot;:&quot;2021-07-18&quot;},&quot;1627102800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 07 24&quot;,&quot;original-date&quot;:&quot;2021-07-24&quot;},&quot;1627189200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 07 25&quot;,&quot;original-date&quot;:&quot;2021-07-25&quot;},&quot;1627707600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 07 31&quot;,&quot;original-date&quot;:&quot;2021-07-31&quot;},&quot;1627794000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 08 01&quot;,&quot;original-date&quot;:&quot;2021-08-01&quot;},&quot;1628312400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 08 07&quot;,&quot;original-date&quot;:&quot;2021-08-07&quot;},&quot;1628398800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 08 08&quot;,&quot;original-date&quot;:&quot;2021-08-08&quot;},&quot;1628917200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 08 14&quot;,&quot;original-date&quot;:&quot;2021-08-14&quot;},&quot;1629003600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 08 15&quot;,&quot;original-date&quot;:&quot;2021-08-15&quot;},&quot;1629522000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 08 21&quot;,&quot;original-date&quot;:&quot;2021-08-21&quot;},&quot;1629608400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 08 22&quot;,&quot;original-date&quot;:&quot;2021-08-22&quot;},&quot;1630126800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 08 28&quot;,&quot;original-date&quot;:&quot;2021-08-28&quot;},&quot;1630213200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 08 29&quot;,&quot;original-date&quot;:&quot;2021-08-29&quot;},&quot;1630731600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 09 04&quot;,&quot;original-date&quot;:&quot;2021-09-04&quot;},&quot;1630818000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 09 05&quot;,&quot;original-date&quot;:&quot;2021-09-05&quot;},&quot;1631336400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 09 11&quot;,&quot;original-date&quot;:&quot;2021-09-11&quot;},&quot;1631422800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 09 12&quot;,&quot;original-date&quot;:&quot;2021-09-12&quot;},&quot;1631941200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 09 18&quot;,&quot;original-date&quot;:&quot;2021-09-18&quot;},&quot;1632027600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 09 19&quot;,&quot;original-date&quot;:&quot;2021-09-19&quot;},&quot;1632546000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 09 25&quot;,&quot;original-date&quot;:&quot;2021-09-25&quot;},&quot;1632632400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 09 26&quot;,&quot;original-date&quot;:&quot;2021-09-26&quot;},&quot;1633150800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 10 02&quot;,&quot;original-date&quot;:&quot;2021-10-02&quot;},&quot;1633237200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 10 03&quot;,&quot;original-date&quot;:&quot;2021-10-03&quot;},&quot;1633755600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 10 09&quot;,&quot;original-date&quot;:&quot;2021-10-09&quot;},&quot;1633842000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 10 10&quot;,&quot;original-date&quot;:&quot;2021-10-10&quot;},&quot;1634360400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 10 16&quot;,&quot;original-date&quot;:&quot;2021-10-16&quot;},&quot;1634446800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 10 17&quot;,&quot;original-date&quot;:&quot;2021-10-17&quot;},&quot;1634965200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 10 23&quot;,&quot;original-date&quot;:&quot;2021-10-23&quot;},&quot;1635051600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 10 24&quot;,&quot;original-date&quot;:&quot;2021-10-24&quot;},&quot;1635570000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 10 30&quot;,&quot;original-date&quot;:&quot;2021-10-30&quot;},&quot;1635656400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 10 31&quot;,&quot;original-date&quot;:&quot;2021-10-31&quot;},&quot;1636174800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 11 06&quot;,&quot;original-date&quot;:&quot;2021-11-06&quot;},&quot;1636261200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 11 07&quot;,&quot;original-date&quot;:&quot;2021-11-07&quot;},&quot;1636783200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 11 13&quot;,&quot;original-date&quot;:&quot;2021-11-13&quot;},&quot;1636869600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 11 14&quot;,&quot;original-date&quot;:&quot;2021-11-14&quot;},&quot;1637388000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 11 20&quot;,&quot;original-date&quot;:&quot;2021-11-20&quot;},&quot;1637474400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 11 21&quot;,&quot;original-date&quot;:&quot;2021-11-21&quot;},&quot;1637992800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 11 27&quot;,&quot;original-date&quot;:&quot;2021-11-27&quot;},&quot;1638079200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 11 28&quot;,&quot;original-date&quot;:&quot;2021-11-28&quot;},&quot;1638597600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 12 04&quot;,&quot;original-date&quot;:&quot;2021-12-04&quot;},&quot;1638684000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 12 05&quot;,&quot;original-date&quot;:&quot;2021-12-05&quot;},&quot;1639202400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 12 11&quot;,&quot;original-date&quot;:&quot;2021-12-11&quot;},&quot;1639288800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 12 12&quot;,&quot;original-date&quot;:&quot;2021-12-12&quot;},&quot;1639807200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 12 18&quot;,&quot;original-date&quot;:&quot;2021-12-18&quot;},&quot;1639893600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 12 19&quot;,&quot;original-date&quot;:&quot;2021-12-19&quot;},&quot;1640412000&quot;:{&quot;type&quot;:&quot;holiday&quot;,&quot;duration&quot;:&quot;0&quot;,&quot;date&quot;:&quot;2021 12 25&quot;,&quot;original-date&quot;:&quot;2021-12-25&quot;},&quot;1640498400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2021 12 26&quot;,&quot;original-date&quot;:&quot;2021-12-26&quot;},&quot;1641016800&quot;:{&quot;type&quot;:&quot;holiday&quot;,&quot;duration&quot;:&quot;0&quot;,&quot;date&quot;:&quot;2022 01 01&quot;,&quot;original-date&quot;:&quot;2022-01-01&quot;},&quot;1641103200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 01 02&quot;,&quot;original-date&quot;:&quot;2022-01-02&quot;},&quot;1641621600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 01 08&quot;,&quot;original-date&quot;:&quot;2022-01-08&quot;},&quot;1641708000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 01 09&quot;,&quot;original-date&quot;:&quot;2022-01-09&quot;},&quot;1642226400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 01 15&quot;,&quot;original-date&quot;:&quot;2022-01-15&quot;},&quot;1642312800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 01 16&quot;,&quot;original-date&quot;:&quot;2022-01-16&quot;},&quot;1642831200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 01 22&quot;,&quot;original-date&quot;:&quot;2022-01-22&quot;},&quot;1642917600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 01 23&quot;,&quot;original-date&quot;:&quot;2022-01-23&quot;},&quot;1643436000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 01 29&quot;,&quot;original-date&quot;:&quot;2022-01-29&quot;},&quot;1643522400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 01 30&quot;,&quot;original-date&quot;:&quot;2022-01-30&quot;},&quot;1644040800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 02 05&quot;,&quot;original-date&quot;:&quot;2022-02-05&quot;},&quot;1644127200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 02 06&quot;,&quot;original-date&quot;:&quot;2022-02-06&quot;},&quot;1644645600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 02 12&quot;,&quot;original-date&quot;:&quot;2022-02-12&quot;},&quot;1644732000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 02 13&quot;,&quot;original-date&quot;:&quot;2022-02-13&quot;},&quot;1645250400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 02 19&quot;,&quot;original-date&quot;:&quot;2022-02-19&quot;},&quot;1645336800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 02 20&quot;,&quot;original-date&quot;:&quot;2022-02-20&quot;},&quot;1645855200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 02 26&quot;,&quot;original-date&quot;:&quot;2022-02-26&quot;},&quot;1645941600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 02 27&quot;,&quot;original-date&quot;:&quot;2022-02-27&quot;},&quot;1646460000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 03 05&quot;,&quot;original-date&quot;:&quot;2022-03-05&quot;},&quot;1646546400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 03 06&quot;,&quot;original-date&quot;:&quot;2022-03-06&quot;},&quot;1647064800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 03 12&quot;,&quot;original-date&quot;:&quot;2022-03-12&quot;},&quot;1647151200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 03 13&quot;,&quot;original-date&quot;:&quot;2022-03-13&quot;},&quot;1647666000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 03 19&quot;,&quot;original-date&quot;:&quot;2022-03-19&quot;},&quot;1647752400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 03 20&quot;,&quot;original-date&quot;:&quot;2022-03-20&quot;},&quot;1648270800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 03 26&quot;,&quot;original-date&quot;:&quot;2022-03-26&quot;},&quot;1648357200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 03 27&quot;,&quot;original-date&quot;:&quot;2022-03-27&quot;},&quot;1648875600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 04 02&quot;,&quot;original-date&quot;:&quot;2022-04-02&quot;},&quot;1648962000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 04 03&quot;,&quot;original-date&quot;:&quot;2022-04-03&quot;},&quot;1649480400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 04 09&quot;,&quot;original-date&quot;:&quot;2022-04-09&quot;},&quot;1649566800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 04 10&quot;,&quot;original-date&quot;:&quot;2022-04-10&quot;},&quot;1650085200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 04 16&quot;,&quot;original-date&quot;:&quot;2022-04-16&quot;},&quot;1650171600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 04 17&quot;,&quot;original-date&quot;:&quot;2022-04-17&quot;},&quot;1650690000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 04 23&quot;,&quot;original-date&quot;:&quot;2022-04-23&quot;},&quot;1650776400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 04 24&quot;,&quot;original-date&quot;:&quot;2022-04-24&quot;},&quot;1651294800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 04 30&quot;,&quot;original-date&quot;:&quot;2022-04-30&quot;},&quot;1651381200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 05 01&quot;,&quot;original-date&quot;:&quot;2022-05-01&quot;},&quot;1651899600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 05 07&quot;,&quot;original-date&quot;:&quot;2022-05-07&quot;},&quot;1651986000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 05 08&quot;,&quot;original-date&quot;:&quot;2022-05-08&quot;},&quot;1652504400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 05 14&quot;,&quot;original-date&quot;:&quot;2022-05-14&quot;},&quot;1652590800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 05 15&quot;,&quot;original-date&quot;:&quot;2022-05-15&quot;},&quot;1653109200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 05 21&quot;,&quot;original-date&quot;:&quot;2022-05-21&quot;},&quot;1653195600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 05 22&quot;,&quot;original-date&quot;:&quot;2022-05-22&quot;},&quot;1653714000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 05 28&quot;,&quot;original-date&quot;:&quot;2022-05-28&quot;},&quot;1653800400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 05 29&quot;,&quot;original-date&quot;:&quot;2022-05-29&quot;},&quot;1654318800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 06 04&quot;,&quot;original-date&quot;:&quot;2022-06-04&quot;},&quot;1654405200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 06 05&quot;,&quot;original-date&quot;:&quot;2022-06-05&quot;},&quot;1654923600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 06 11&quot;,&quot;original-date&quot;:&quot;2022-06-11&quot;},&quot;1655010000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 06 12&quot;,&quot;original-date&quot;:&quot;2022-06-12&quot;},&quot;1655528400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 06 18&quot;,&quot;original-date&quot;:&quot;2022-06-18&quot;},&quot;1655614800&quot;:{&quot;type&quot;:&quot;holiday&quot;,&quot;duration&quot;:&quot;0&quot;,&quot;date&quot;:&quot;2022 06 19&quot;,&quot;original-date&quot;:&quot;2022-06-19&quot;},&quot;1656133200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 06 25&quot;,&quot;original-date&quot;:&quot;2022-06-25&quot;},&quot;1656219600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 06 26&quot;,&quot;original-date&quot;:&quot;2022-06-26&quot;},&quot;1656738000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 07 02&quot;,&quot;original-date&quot;:&quot;2022-07-02&quot;},&quot;1656824400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 07 03&quot;,&quot;original-date&quot;:&quot;2022-07-03&quot;},&quot;1657342800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 07 09&quot;,&quot;original-date&quot;:&quot;2022-07-09&quot;},&quot;1657429200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 07 10&quot;,&quot;original-date&quot;:&quot;2022-07-10&quot;},&quot;1657947600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 07 16&quot;,&quot;original-date&quot;:&quot;2022-07-16&quot;},&quot;1658034000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 07 17&quot;,&quot;original-date&quot;:&quot;2022-07-17&quot;},&quot;1658552400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 07 23&quot;,&quot;original-date&quot;:&quot;2022-07-23&quot;},&quot;1658638800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 07 24&quot;,&quot;original-date&quot;:&quot;2022-07-24&quot;},&quot;1659157200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 07 30&quot;,&quot;original-date&quot;:&quot;2022-07-30&quot;},&quot;1659243600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 07 31&quot;,&quot;original-date&quot;:&quot;2022-07-31&quot;},&quot;1659762000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 08 06&quot;,&quot;original-date&quot;:&quot;2022-08-06&quot;},&quot;1659848400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 08 07&quot;,&quot;original-date&quot;:&quot;2022-08-07&quot;},&quot;1660366800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 08 13&quot;,&quot;original-date&quot;:&quot;2022-08-13&quot;},&quot;1660453200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 08 14&quot;,&quot;original-date&quot;:&quot;2022-08-14&quot;},&quot;1660971600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 08 20&quot;,&quot;original-date&quot;:&quot;2022-08-20&quot;},&quot;1661058000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 08 21&quot;,&quot;original-date&quot;:&quot;2022-08-21&quot;},&quot;1661576400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 08 27&quot;,&quot;original-date&quot;:&quot;2022-08-27&quot;},&quot;1661662800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 08 28&quot;,&quot;original-date&quot;:&quot;2022-08-28&quot;},&quot;1662181200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 09 03&quot;,&quot;original-date&quot;:&quot;2022-09-03&quot;},&quot;1662267600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 09 04&quot;,&quot;original-date&quot;:&quot;2022-09-04&quot;},&quot;1662786000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 09 10&quot;,&quot;original-date&quot;:&quot;2022-09-10&quot;},&quot;1662872400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 09 11&quot;,&quot;original-date&quot;:&quot;2022-09-11&quot;},&quot;1663390800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 09 17&quot;,&quot;original-date&quot;:&quot;2022-09-17&quot;},&quot;1663477200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 09 18&quot;,&quot;original-date&quot;:&quot;2022-09-18&quot;},&quot;1663995600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 09 24&quot;,&quot;original-date&quot;:&quot;2022-09-24&quot;},&quot;1664082000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 09 25&quot;,&quot;original-date&quot;:&quot;2022-09-25&quot;},&quot;1664600400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 10 01&quot;,&quot;original-date&quot;:&quot;2022-10-01&quot;},&quot;1664686800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 10 02&quot;,&quot;original-date&quot;:&quot;2022-10-02&quot;},&quot;1665205200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 10 08&quot;,&quot;original-date&quot;:&quot;2022-10-08&quot;},&quot;1665291600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 10 09&quot;,&quot;original-date&quot;:&quot;2022-10-09&quot;},&quot;1665810000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 10 15&quot;,&quot;original-date&quot;:&quot;2022-10-15&quot;},&quot;1665896400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 10 16&quot;,&quot;original-date&quot;:&quot;2022-10-16&quot;},&quot;1666414800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 10 22&quot;,&quot;original-date&quot;:&quot;2022-10-22&quot;},&quot;1666501200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 10 23&quot;,&quot;original-date&quot;:&quot;2022-10-23&quot;},&quot;1667019600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 10 29&quot;,&quot;original-date&quot;:&quot;2022-10-29&quot;},&quot;1667106000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 10 30&quot;,&quot;original-date&quot;:&quot;2022-10-30&quot;},&quot;1667624400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 11 05&quot;,&quot;original-date&quot;:&quot;2022-11-05&quot;},&quot;1667710800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 11 06&quot;,&quot;original-date&quot;:&quot;2022-11-06&quot;},&quot;1668232800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 11 12&quot;,&quot;original-date&quot;:&quot;2022-11-12&quot;},&quot;1668319200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 11 13&quot;,&quot;original-date&quot;:&quot;2022-11-13&quot;},&quot;1668837600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 11 19&quot;,&quot;original-date&quot;:&quot;2022-11-19&quot;},&quot;1668924000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 11 20&quot;,&quot;original-date&quot;:&quot;2022-11-20&quot;},&quot;1669442400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 11 26&quot;,&quot;original-date&quot;:&quot;2022-11-26&quot;},&quot;1669528800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 11 27&quot;,&quot;original-date&quot;:&quot;2022-11-27&quot;},&quot;1670047200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 12 03&quot;,&quot;original-date&quot;:&quot;2022-12-03&quot;},&quot;1670133600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 12 04&quot;,&quot;original-date&quot;:&quot;2022-12-04&quot;},&quot;1670652000&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 12 10&quot;,&quot;original-date&quot;:&quot;2022-12-10&quot;},&quot;1670738400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 12 11&quot;,&quot;original-date&quot;:&quot;2022-12-11&quot;},&quot;1671256800&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 12 17&quot;,&quot;original-date&quot;:&quot;2022-12-17&quot;},&quot;1671343200&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 12 18&quot;,&quot;original-date&quot;:&quot;2022-12-18&quot;},&quot;1671861600&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 12 24&quot;,&quot;original-date&quot;:&quot;2022-12-24&quot;},&quot;1671948000&quot;:{&quot;type&quot;:&quot;holiday&quot;,&quot;duration&quot;:&quot;0&quot;,&quot;date&quot;:&quot;2022 12 25&quot;,&quot;original-date&quot;:&quot;2022-12-25&quot;},&quot;1672466400&quot;:{&quot;type&quot;:&quot;weekend&quot;,&quot;duration&quot;:&quot;8&quot;,&quot;date&quot;:&quot;2022 12 31&quot;,&quot;original-date&quot;:&quot;2022-12-31&quot;},&quot;1577858400&quot;:{&quot;type&quot;:&quot;holiday&quot;,&quot;duration&quot;:&quot;0&quot;,&quot;date&quot;:&quot;2020 01 01&quot;,&quot;original-date&quot;:&quot;2020-01-01&quot;},&quot;1579500000&quot;:{&quot;type&quot;:&quot;holiday&quot;,&quot;duration&quot;:&quot;0&quot;,&quot;date&quot;:&quot;2020 01 20&quot;,&quot;original-date&quot;:&quot;2020-01-20&quot;},&quot;1581919200&quot;:{&quot;type&quot;:&quot;holiday&quot;,&quot;duration&quot;:&quot;0&quot;,&quot;date&quot;:&quot;2020 02 17&quot;,&quot;original-date&quot;:&quot;2020-02-17&quot;},&quot;1590382800&quot;:{&quot;type&quot;:&quot;holiday&quot;,&quot;duration&quot;:&quot;0&quot;,&quot;date&quot;:&quot;2020 05 25&quot;,&quot;original-date&quot;:&quot;2020-05-25&quot;},&quot;1592542800&quot;:{&quot;type&quot;:&quot;holiday&quot;,&quot;duration&quot;:&quot;0&quot;,&quot;date&quot;:&quot;2020 06 19&quot;,&quot;original-date&quot;:&quot;2020-06-19&quot;},&quot;1599454800&quot;:{&quot;type&quot;:&quot;holiday&quot;,&quot;duration&quot;:&quot;0&quot;,&quot;date&quot;:&quot;2020 09 07&quot;,&quot;original-date&quot;:&quot;2020-09-07&quot;},&quot;1602478800&quot;:{&quot;type&quot;:&quot;holiday&quot;,&quot;duration&quot;:&quot;0&quot;,&quot;date&quot;:&quot;2020 10 12&quot;,&quot;original-date&quot;:&quot;2020-10-12&quot;},&quot;1605074400&quot;:{&quot;type&quot;:&quot;holiday&quot;,&quot;duration&quot;:&quot;0&quot;,&quot;date&quot;:&quot;2020 11 11&quot;,&quot;original-date&quot;:&quot;2020-11-11&quot;},&quot;1606370400&quot;:{&quot;type&quot;:&quot;holiday&quot;,&quot;duration&quot;:&quot;0&quot;,&quot;date&quot;:&quot;2020 11 26&quot;,&quot;original-date&quot;:&quot;2020-11-26&quot;},&quot;1608876000&quot;:{&quot;type&quot;:&quot;holiday&quot;,&quot;duration&quot;:&quot;0&quot;,&quot;date&quot;:&quot;2020 12 25&quot;,&quot;original-date&quot;:&quot;2020-12-25&quot;},&quot;1609480800&quot;:{&quot;type&quot;:&quot;holiday&quot;,&quot;duration&quot;:&quot;0&quot;,&quot;date&quot;:&quot;2021 01 01&quot;,&quot;original-date&quot;:&quot;2021-01-01&quot;},&quot;1636610400&quot;:{&quot;type&quot;:&quot;holiday&quot;,&quot;duration&quot;:&quot;0&quot;,&quot;date&quot;:&quot;2021 11 11&quot;,&quot;original-date&quot;:&quot;2021-11-11&quot;},&quot;1656910800&quot;:{&quot;type&quot;:&quot;holiday&quot;,&quot;duration&quot;:&quot;0&quot;,&quot;date&quot;:&quot;2022 07 04&quot;,&quot;original-date&quot;:&quot;2022-07-04&quot;},&quot;1668146400&quot;:{&quot;type&quot;:&quot;holiday&quot;,&quot;duration&quot;:&quot;0&quot;,&quot;date&quot;:&quot;2022 11 11&quot;,&quot;original-date&quot;:&quot;2022-11-11&quot;}};
                    highlightOffDays(this, offDays);
                    
                    fixDropDown(this.$root.find('.picker__select--year'));
                    fixDropDown(this.$root.find('.picker__select--month'));
                }
             });
             function highlightOffDays(pickerInstance, offDays) {
                    var year = pickerInstance.$root.find('select.picker__select--year').val();
                    if (year === undefined) {
                        year = pickerInstance.$root.find('div.picker__year').html();
                    }
                    var month = parseInt(pickerInstance.$root.find('select.picker__select--month').val(), 10) + 1;

                    if (month &lt; 10) {
                        month = '0' + month;
                    }

                    if (offDays != null) {
                        _.forEach(offDays, function (value, key) {
                            if (value.date.indexOf(year + ' ' + month) !== -1) {
                                var applyClass = 'pickadate--' + value.type;
                                if (value.duration == 4) {
                                    applyClass += '-half';
                                }
                                // $('[data-pick=' + key + '000]').addClass( applyClass).addClass('circle');
                                $('[data-pick=' + moment(value.date, 'YYYY MM DD').toDate().getTime() + ']').addClass(applyClass);
                            }
                        });
                    }
                }
                
                function fixDropDown(element) {
                    if (!element.hasClass('initialized') &amp;&amp; !element.is(':disabled')) {


                        element.removeClass('browser-default').material_select(function () {
                            element.siblings('input.select-dropdown').trigger('close');
                        });

                        var onMouseDown = function (e) {
                            if (e.clientX >= e.target.clientWidth || e.clientY >= e.target.clientHeight) {
                                e.preventDefault();
                            }
                        };
                        
                        var onMouseClick = function () {
                            if($(this).closest(&quot;div.select-wrapper&quot;).length){
                                var dropDown = $(this).closest(&quot;div.select-wrapper&quot;).find(&quot;ul.select-dropdown&quot;);
                                dropDown.find('li:nth-child('+parseInt(dropDown.find('li').length/2 + 1)+')')[0].scrollIntoView();
                            }
                        };

                        element.siblings('input.select-dropdown').on('click', onMouseClick);                        
                        element.siblings('input.select-dropdown').on('mousedown', onMouseDown);
                    }
                }
                
                                       Date of Application                        
                    

                                            
                            
                                                                Facebook                            
                            
                                                                Twitter                            
                            
                                                                LinkedIn                            
                        
                                        
                        
                            Application Received
                             
                            
                        
                Application Received
        
                        
                Shortlisted
        
                        
                Interview
        
                        
                Job Offer
        
                        
                Hired
        
                        
                Rejected
        $(&quot;.dropdown-employees&quot;).dropdown({
            inDuration: 300,
            outDuration: 225,
            constrain_width: false, 
            hover: false, 
            gutter: 200, 
            belowOrigin: false
          });                            Current Stage of Recruitment                        
                    
                    
                        
                                                        Keywords                        
                    

                    
                        
                                                        Fatima Zahrae's comment                        
                    

                    
                        
                            
                        
                    
                    
                                            
                    
                        * Required field                    
                    
                                                                                Save
                                                                        
                
                
                    Job Description                        
                    
                        Software Engineer
                    
                
            
        
    





    
        
        Changing the vacancy may result in loss of vacancy specific data. Are you sure you want to continue?
    

    
        
            No, Cancel        
        
            Yes, Continue        
    






    
        
        You are about to delete data permanently. Are you sure you want to continue?
    

    
        
            no, cancel        
        
            yes, delete        
    




    var msgSuccessfullySaved = 'Successfully Saved';
    var vacancyId = '59';
    var candidateId = '150';
    var candidateProfileUrl = '/recruitment/candidateProfile?candidateId=150&amp;vacancyId=#VACANCY_ID_URL_PARAM#';
    var saveCandidateDetailsUrl = '/recruitment/addCandidate/id/150';
    var saveApplicationFormUrl = '/recruitment/viewQuestionForm/candidateId/150/vacancyId/59';
    var getVacancyDataByVacancyIdAjaxUrl = '/recruitment/getVacancyDataByVacancyIdAjax';
    var deleteAttachmentAnswerAjaxUrl = '/recruitment/deleteAttachmentAnswerAjax';
    var vacancyStatus = &quot;NEW&quot;;
    var lang_required = 'Required';
    var lang_validEmail = 'Expected format: admin@example.com';
    var lang_tooLargeInput = &quot;Should not exceed 30 characters&quot;;
    var lang_fileSizeInvalid = &quot;File size should be less than 2MB&quot;;
    var lang_noMoreThan254 = &quot;Should not exceed 254 characters&quot;;
    var lang_noMoreThan255 = &quot;Should not exceed 255 characters&quot;;
    var lang_tooLarge65535 = &quot;Should not exceed 65535 characters&quot;;
    var lang_validPhoneNo = &quot;Allows numbers and only + - / ( )&quot;;
    var lang_fileTypeInvalid = &quot;File type not allowed&quot;;
    var localizationDateFormat = &quot;D, d M Y&quot;;
    var lang_invalidSocialUrl = 'Should be a valid url';
    var lang_noMoreThan100 = &quot;Should not exceed 100 characters&quot;;
    var application_form = &quot;/recruitment/exportCandidateApplicationFormPdf?candidateId=150&quot;
    var lang_deleteSuccess = 'Successfully Deleted';
    var textLengthExceedsSectionName = &quot;Should not exceed 255 characters&quot;;

     
    
    
        


    Candidate Notes
            
            
                                
                    
                    Enter note about Fatima Zahrae Sen
                        Save
                
    
            
        
        




    
    






    var msgSuccessfullySaved = 'Successfully Saved';
    var msgSuccessfullyDeleted = 'Successfully Deleted';
    var lang_noMoreThan255 = &quot;Should not exceed 255 characters&quot;;
    var lang_tooLarge65535 = &quot;Should not exceed 65535 characters&quot;;

    

    
            

</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;candidate-profile-div&quot;)</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='candidate-profile-div']</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='content']/div[2]/div</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Help'])[1]/following::div[2]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='help'])[1]/following::div[2]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div/div/div/div[2]/div</value>
   </webElementXpaths>
</WebElementEntity>
