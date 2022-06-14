<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>0e9ec370-2118-42cd-8ef2-c7a93c47939b</elementGuidId>
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
      <value>http://localhost:8080/kestrel/SavedTripDeleteService</value>
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
      &lt;univ:SupportedVersions urVersion=&quot;ac vinclis et&quot; airVersion=&quot;rapidum iaculata e&quot; hotelVersion=&quot;caelumque profundum&quot; vehicleVersion=&quot;circum claustra&quot; passiveVersion=&quot;coniunx una&quot; railVersion=&quot;arce sceptra&quot; cruiseVersion=&quot;circum claustra fremunt&quot;/>
   &lt;/soapenv:Header>
   &lt;soapenv:Body>
      &lt;univ:SavedTripDeleteReq TraceId=&quot;gero et&quot; TokenId=&quot;sonoras imperio&quot; AuthorizedBy=&quot;quae divum incedo&quot; TargetBranch=&quot;verrantque per auras&quot; OverrideLogging=&quot;ERROR&quot; LanguageCode=&quot;turbine corripuit&quot; RetrieveProviderReservationDetails=&quot;false&quot; LocatorCode=&quot;regemque&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;fremunt celsa&quot; CIDBNumber=&quot;100&quot;/>
         &lt;com:AgentIDOverride SupplierCode=&quot;feran&quot; ProviderCode=&quot;scept&quot; AgentID=&quot;turbine corripuit scopuloque&quot;/>
         &lt;com:TerminalSessionInfo>flammato secum dea&lt;/com:TerminalSessionInfo>
         &lt;com:OverridePCC ProviderCode=&quot;profu&quot; PseudoCityCode=&quot;nubibus ig&quot;/>
      &lt;/univ:SavedTripDeleteReq>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://localhost:8080/kestrel/SavedTripDeleteService</soapServiceEndpoint>
   <soapServiceFunction>service</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <wsdlAddress>file:/S:/TestData/UAPI_JAVA_Framework/wsdl/universal_v51_0/UniversalRecord.wsdl</wsdlAddress>
</WebServiceRequestEntity>
