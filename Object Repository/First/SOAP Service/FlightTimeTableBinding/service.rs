<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>eccee53d-87c0-4018-ace4-3f196c5e57e1</elementGuidId>
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
      <value>http://localhost:8080/kestrel/AirService</value>
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
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:air=&quot;http://www.travelport.com/schema/air_v51_0&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v51_0&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;air:FlightTimeTableReq TraceId=&quot;gero et&quot; TokenId=&quot;sonoras imperio&quot; AuthorizedBy=&quot;quae divum incedo&quot; TargetBranch=&quot;verrantque per auras&quot; OverrideLogging=&quot;ERROR&quot; LanguageCode=&quot;turbine corripuit&quot; RetrieveProviderReservationDetails=&quot;false&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;regemque dedit&quot; CIDBNumber=&quot;100&quot;/>
         &lt;com:AgentIDOverride SupplierCode=&quot;fremu&quot; ProviderCode=&quot;feran&quot; AgentID=&quot;sceptra tenens&quot;/>
         &lt;com:TerminalSessionInfo>turbine corripuit scopuloque&lt;/com:TerminalSessionInfo>
         &lt;com:OverridePCC ProviderCode=&quot;flamm&quot; PseudoCityCode=&quot;profundum &quot;/>
         &lt;com:NextResultReference ProviderCode=&quot;nubib&quot;>ac vinclis et&lt;/com:NextResultReference>
         &lt;air:FlightTimeTableCriteria>
            &lt;air:GeneralTimeTable StartDate=&quot;rapidum iaculata e&quot; EndDate=&quot;caelumque profundum&quot; StartTime=&quot;13:26:46+05:30&quot; EndTime=&quot;11:48:09&quot; IncludeConnection=&quot;false&quot;>
               &lt;air:DaysOfOperation Mon=&quot;false&quot; Tue=&quot;true&quot; Wed=&quot;false&quot; Thu=&quot;true&quot; Fri=&quot;true&quot; Sat=&quot;false&quot; Sun=&quot;false&quot;/>
               &lt;air:FlightOrigin>
                  &lt;com:Airport Code=&quot;sed&quot;/>
                  &lt;com:City Code=&quot;tal&quot;/>
                  &lt;com:CityOrAirport Code=&quot;aus&quot; PreferCity=&quot;false&quot;/>
               &lt;/air:FlightOrigin>
               &lt;air:FlightDestination>
                  &lt;com:Airport Code=&quot;sci&quot;/>
                  &lt;com:City Code=&quot;dar&quot;/>
                  &lt;com:CityOrAirport Code=&quot;bel&quot; PreferCity=&quot;false&quot;/>
               &lt;/air:FlightDestination>
               &lt;air:CarrierList IncludeCarrier=&quot;true&quot;>
                  &lt;air:CarrierCode>fa&lt;/air:CarrierCode>
               &lt;/air:CarrierList>
            &lt;/air:GeneralTimeTable>
            &lt;air:SpecificTimeTable StartDate=&quot;et quisquam&quot; Carrier=&quot;et&quot; FlightNumber=&quot;annos&quot;>
               &lt;air:FlightOrigin>
                  &lt;com:Airport Code=&quot;ven&quot;/>
               &lt;/air:FlightOrigin>
               &lt;air:FlightDestination>
                  &lt;com:Airport Code=&quot;reg&quot;/>
               &lt;/air:FlightDestination>
            &lt;/air:SpecificTimeTable>
         &lt;/air:FlightTimeTableCriteria>
      &lt;/air:FlightTimeTableReq>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://localhost:8080/kestrel/AirService</soapServiceEndpoint>
   <soapServiceFunction>service</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <wsdlAddress>file:/S:/TestData/UAPI_JAVA_Framework/wsdl/air_v51_0/Air.wsdl</wsdlAddress>
</WebServiceRequestEntity>
