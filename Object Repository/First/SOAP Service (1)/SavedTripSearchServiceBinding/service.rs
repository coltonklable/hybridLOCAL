<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>b9644abc-90a5-4b86-b3f5-5d13c6c1b14d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>SOAPAction</name>
      <type>Main</type>
      <value>http://localhost:8080/kestrel/SavedTripSearchService</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:univ=&quot;http://www.travelport.com/schema/universal_v51_0&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v51_0&quot;>
   &lt;soapenv:Header>
      &lt;univ:SupportedVersions urVersion=&quot;aris imponet honorem&quot; airVersion=&quot;molemque et&quot; hotelVersion=&quot;rapidum iaculata e&quot; vehicleVersion=&quot;nimborum in&quot; passiveVersion=&quot;altos imposuit regemque&quot; railVersion=&quot;aeoliam venit&quot; cruiseVersion=&quot;hic vasto&quot;/>
   &lt;/soapenv:Header>
   &lt;soapenv:Body>
      &lt;univ:SavedTripSearchReq TraceId=&quot;gero et&quot; TokenId=&quot;sonoras imperio&quot; AuthorizedBy=&quot;quae divum incedo&quot; TargetBranch=&quot;verrantque per auras&quot; OverrideLogging=&quot;ERROR&quot; LanguageCode=&quot;turbine corripuit&quot; RetrieveProviderReservationDetails=&quot;false&quot; RecordStatus=&quot;Past&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;nimborum in&quot; CIDBNumber=&quot;100&quot;/>
         &lt;com:AgentIDOverride SupplierCode=&quot;foede&quot; ProviderCode=&quot;profu&quot; AgentID=&quot;et carcere&quot;/>
         &lt;com:TerminalSessionInfo>iovis rapidum iaculata&lt;/com:TerminalSessionInfo>
         &lt;com:OverridePCC ProviderCode=&quot;spelu&quot; PseudoCityCode=&quot;bella gero&quot;/>
         &lt;univ:SavedTripSearchModifiers IncludeAllNames=&quot;false&quot; IncludeAgentInfo=&quot;false&quot; MaxResults=&quot;20&quot; StartFromResult=&quot;100&quot; ExcludeAir=&quot;false&quot; ExcludeVehicle=&quot;false&quot; ExcludeHotel=&quot;false&quot; SavedTripName=&quot;flammas turbine&quot; ExcludeURAssociated=&quot;true&quot;>
            &lt;univ:TravelDate>
               &lt;univ:DateRange StartDate=&quot;2002-11-16+05:30&quot; EndDate=&quot;2000-09-14&quot;/>
               &lt;univ:SpecificDate>2015-08-24&lt;/univ:SpecificDate>
            &lt;/univ:TravelDate>
         &lt;/univ:SavedTripSearchModifiers>
         &lt;univ:TravelerCriteria PhoneNumber=&quot;arce sceptra&quot; VIPOnly=&quot;false&quot;>
            &lt;univ:NameCriteria FirstName=&quot;imperio premit&quot; LastName=&quot;quisquam numen&quot;/>
            &lt;univ:AppliedProfileCriteria TravelerProfileId=&quot;100&quot;/>
         &lt;/univ:TravelerCriteria>
         &lt;univ:SearchAgent CreatedByAgent=&quot;ac vinclis&quot; ModifiedByAgent=&quot;ac vinclis&quot; TicketedByAgent=&quot;pectore flammas&quot;>
            &lt;univ:BranchId>pectore flammas&lt;/univ:BranchId>
         &lt;/univ:SearchAgent>
         &lt;univ:AirReservationCriteria Origin=&quot;ann&quot; Destination=&quot;cer&quot; FlightNumber=&quot;rates evertitque aequora&quot; Carrier=&quot;vo&quot; PassiveOnly=&quot;false&quot;>
            &lt;univ:DepartureDate>
               &lt;univ:DateRange StartDate=&quot;2011-06-04&quot; EndDate=&quot;2018-01-21+05:30&quot;/>
               &lt;univ:SpecificDate>2014-12-08&lt;/univ:SpecificDate>
            &lt;/univ:DepartureDate>
            &lt;univ:ArrivalDate>
               &lt;univ:DateRange StartDate=&quot;2019-03-02&quot; EndDate=&quot;2018-04-14&quot;/>
               &lt;univ:SpecificDate>2012-01-08+05:30&lt;/univ:SpecificDate>
            &lt;/univ:ArrivalDate>
         &lt;/univ:AirReservationCriteria>
         &lt;univ:HotelReservationCriteria HotelChainCode=&quot;in&quot; HotelCode=&quot;abdidit atris hoc&quot; HotelConfirmation=&quot;coniunx una cum&quot; Location=&quot;ips&quot; PassiveOnly=&quot;false&quot;>
            &lt;univ:CheckInDate>
               &lt;univ:DateRange StartDate=&quot;2015-07-31+05:30&quot; EndDate=&quot;2017-08-26&quot;/>
               &lt;univ:SpecificDate>2000-02-21+05:30&lt;/univ:SpecificDate>
            &lt;/univ:CheckInDate>
         &lt;/univ:HotelReservationCriteria>
         &lt;univ:VehicleReservationCriteria VehicleConfirmation=&quot;ferant rapidi secum&quot; Location=&quot;nim&quot; VendorCode=&quot;vasto&quot; LocationNumber=&quot;flammato secum&quot; PassiveOnly=&quot;false&quot;>
            &lt;univ:PickUpDate>
               &lt;univ:DateRange StartDate=&quot;2014-10-15&quot; EndDate=&quot;2003-02-28+05:30&quot;/>
               &lt;univ:SpecificDate>2007-06-23+05:30&lt;/univ:SpecificDate>
            &lt;/univ:PickUpDate>
         &lt;/univ:VehicleReservationCriteria>
         &lt;univ:RailReservationCriteria JourneyOrigin=&quot;et &quot; JourneyDestination=&quot;ded&quot; JourneyRailLocOrigin=&quot;profundu&quot; JourneyRailLocDestination=&quot;imposuit&quot; SupplierCode=&quot;magno cum&quot; TrainNumber=&quot;iras ni faciat&quot; PassiveOnly=&quot;false&quot;>
            &lt;univ:JourneyDepartureDate>
               &lt;univ:DateRange StartDate=&quot;2016-10-25+05:30&quot; EndDate=&quot;2014-01-07&quot;/>
               &lt;univ:SpecificDate>2011-09-24+05:30&lt;/univ:SpecificDate>
            &lt;/univ:JourneyDepartureDate>
            &lt;univ:JourneyArrivalDate>
               &lt;univ:DateRange StartDate=&quot;2015-07-12&quot; EndDate=&quot;2011-01-01+05:30&quot;/>
               &lt;univ:SpecificDate>2008-11-04&lt;/univ:SpecificDate>
            &lt;/univ:JourneyArrivalDate>
            &lt;univ:SegmentDepartureDate>
               &lt;univ:DateRange StartDate=&quot;2002-11-30&quot; EndDate=&quot;2019-08-10&quot;/>
               &lt;univ:SpecificDate>2003-04-21+05:30&lt;/univ:SpecificDate>
            &lt;/univ:SegmentDepartureDate>
            &lt;univ:SegmentArrivalDate>
               &lt;univ:DateRange StartDate=&quot;2003-03-28+05:30&quot; EndDate=&quot;2015-12-02&quot;/>
               &lt;univ:SpecificDate>2007-10-06&lt;/univ:SpecificDate>
            &lt;/univ:SegmentArrivalDate>
         &lt;/univ:RailReservationCriteria>
      &lt;/univ:SavedTripSearchReq>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://localhost:8080/kestrel/SavedTripSearchService</soapServiceEndpoint>
   <soapServiceFunction>service</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <wsdlAddress>file:/S:/TestData/UAPI_JAVA_Framework/wsdl/universal_v51_0/UniversalRecord.wsdl</wsdlAddress>
</WebServiceRequestEntity>
