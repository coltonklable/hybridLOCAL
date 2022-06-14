<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>562ee316-a59c-4d46-93ca-489cd723e461</elementGuidId>
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
      <value>http://localhost:8080/kestrel/UniversalRecordService</value>
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
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;univ:UniversalRecordSearchReq TraceId=&quot;gero et&quot; TokenId=&quot;sonoras imperio&quot; AuthorizedBy=&quot;quae divum incedo&quot; TargetBranch=&quot;verrantque per auras&quot; OverrideLogging=&quot;ERROR&quot; LanguageCode=&quot;turbine corripuit&quot; RetrieveProviderReservationDetails=&quot;false&quot; ActionDate=&quot;2017-05-15&quot; RecordStatus=&quot;Current&quot; ClientId=&quot;ferant rapidi&quot; ProviderCode=&quot;sceptra tenens&quot; ProviderLocatorCode=&quot;turbine corripuit scopuloque&quot; ExternalSearchIndex=&quot;flammato secum dea&quot; RestrictToProfileId=&quot;anySimpleType&quot; PassiveOnly=&quot;false&quot; TicketStatus=&quot;Ticketed&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;bella gero et&quot; CIDBNumber=&quot;100&quot;/>
         &lt;com:AgentIDOverride SupplierCode=&quot;flamm&quot; ProviderCode=&quot;hoc m&quot; AgentID=&quot;ac vinclis&quot;/>
         &lt;com:TerminalSessionInfo>speluncis abdidit&lt;/com:TerminalSessionInfo>
         &lt;com:OverridePCC ProviderCode=&quot;aris &quot; PseudoCityCode=&quot;praeterea &quot;/>
         &lt;univ:UniversalRecordSearchModifiers IncludeAllNames=&quot;false&quot; IncludeAgentInfo=&quot;false&quot; MaxResults=&quot;20&quot; StartFromResult=&quot;100&quot; ExcludeAir=&quot;false&quot; ExcludeVehicle=&quot;false&quot; ExcludeHotel=&quot;false&quot;>
            &lt;univ:TravelDate>
               &lt;univ:DateRange StartDate=&quot;2014-06-27-06:00&quot; EndDate=&quot;2017-08-16&quot;/>
               &lt;univ:SpecificDate>2012-04-24-06:00&lt;/univ:SpecificDate>
            &lt;/univ:TravelDate>
         &lt;/univ:UniversalRecordSearchModifiers>
         &lt;univ:TravelerCriteria PhoneNumber=&quot;sed pater&quot; VIPOnly=&quot;false&quot;>
            &lt;univ:NameCriteria FirstName=&quot;pectore flammas&quot; LastName=&quot;annos bella gero&quot;/>
            &lt;univ:AppliedProfileCriteria TravelerProfileId=&quot;100&quot;/>
         &lt;/univ:TravelerCriteria>
         &lt;univ:SearchAgent CreatedByAgent=&quot;certo et&quot; ModifiedByAgent=&quot;rates evertitque aequora&quot; TicketedByAgent=&quot;volutans nimborum in&quot;>
            &lt;univ:BranchId>faciat maria&lt;/univ:BranchId>
         &lt;/univ:SearchAgent>
         &lt;univ:AirReservationCriteria Origin=&quot;et &quot; Destination=&quot;et &quot; FlightNumber=&quot;annos bella gero&quot; Carrier=&quot;ve&quot; PassiveOnly=&quot;false&quot;>
            &lt;univ:DepartureDate>
               &lt;univ:DateRange StartDate=&quot;2016-10-07-06:00&quot; EndDate=&quot;2002-11-05&quot;/>
               &lt;univ:SpecificDate>2011-11-02&lt;/univ:SpecificDate>
            &lt;/univ:DepartureDate>
            &lt;univ:ArrivalDate>
               &lt;univ:DateRange StartDate=&quot;2008-08-28-06:00&quot; EndDate=&quot;2008-11-06-07:00&quot;/>
               &lt;univ:SpecificDate>2019-05-03&lt;/univ:SpecificDate>
            &lt;/univ:ArrivalDate>
         &lt;/univ:AirReservationCriteria>
         &lt;univ:HotelReservationCriteria HotelChainCode=&quot;ni&quot; HotelCode=&quot;mollitque animos&quot; HotelConfirmation=&quot;montis insuper altos&quot; Location=&quot;mol&quot; PassiveOnly=&quot;false&quot;>
            &lt;univ:CheckInDate>
               &lt;univ:DateRange StartDate=&quot;2001-01-03&quot; EndDate=&quot;2014-01-10&quot;/>
               &lt;univ:SpecificDate>2015-07-10&lt;/univ:SpecificDate>
            &lt;/univ:CheckInDate>
         &lt;/univ:HotelReservationCriteria>
         &lt;univ:VehicleReservationCriteria VehicleConfirmation=&quot;tenens mollitque&quot; Location=&quot;pre&quot; VendorCode=&quot;iras &quot; LocationNumber=&quot;nubibus ignem&quot; PassiveOnly=&quot;false&quot;>
            &lt;univ:PickUpDate>
               &lt;univ:DateRange StartDate=&quot;2017-11-22&quot; EndDate=&quot;2005-07-18&quot;/>
               &lt;univ:SpecificDate>2008-12-01&lt;/univ:SpecificDate>
            &lt;/univ:PickUpDate>
         &lt;/univ:VehicleReservationCriteria>
         &lt;univ:RailReservationCriteria JourneyOrigin=&quot;ira&quot; JourneyDestination=&quot;ver&quot; JourneyRailLocOrigin=&quot;metuens &quot; JourneyRailLocDestination=&quot;dare ius&quot; SupplierCode=&quot;tempestatesque sonoras imperio&quot; TrainNumber=&quot;sceptra tenens mollitque&quot; PassiveOnly=&quot;false&quot;>
            &lt;univ:JourneyDepartureDate>
               &lt;univ:DateRange StartDate=&quot;2016-10-16-06:00&quot; EndDate=&quot;2016-04-18-06:00&quot;/>
               &lt;univ:SpecificDate>2008-07-28&lt;/univ:SpecificDate>
            &lt;/univ:JourneyDepartureDate>
            &lt;univ:JourneyArrivalDate>
               &lt;univ:DateRange StartDate=&quot;2003-02-11-07:00&quot; EndDate=&quot;2015-12-10&quot;/>
               &lt;univ:SpecificDate>2014-08-11-06:00&lt;/univ:SpecificDate>
            &lt;/univ:JourneyArrivalDate>
            &lt;univ:SegmentDepartureDate>
               &lt;univ:DateRange StartDate=&quot;2017-09-09-06:00&quot; EndDate=&quot;2017-02-08&quot;/>
               &lt;univ:SpecificDate>2017-05-18&lt;/univ:SpecificDate>
            &lt;/univ:SegmentDepartureDate>
            &lt;univ:SegmentArrivalDate>
               &lt;univ:DateRange StartDate=&quot;2017-07-13&quot; EndDate=&quot;2006-02-20-07:00&quot;/>
               &lt;univ:SpecificDate>2008-09-09-06:00&lt;/univ:SpecificDate>
            &lt;/univ:SegmentArrivalDate>
         &lt;/univ:RailReservationCriteria>
         &lt;univ:SearchAccount ClientID=&quot;aeoliam venit&quot; BranchID=&quot;hic vasto&quot;/>
      &lt;/univ:UniversalRecordSearchReq>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://localhost:8080/kestrel/UniversalRecordService</soapServiceEndpoint>
   <soapServiceFunction>service</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <wsdlAddress>file:/S:/TestData/UAPI_JAVA_Framework/wsdl/universal_v51_0/UniversalRecord.wsdl</wsdlAddress>
</WebServiceRequestEntity>
