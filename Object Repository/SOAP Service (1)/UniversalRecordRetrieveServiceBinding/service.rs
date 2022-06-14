<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>ae60b889-3839-4f14-9e12-2775faac933b</elementGuidId>
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
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:univ=&quot;http://www.travelport.com/schema/universal_v51_0&quot; xmlns:ses=&quot;http://www.travelport.com/soa/common/security/SessionContext_v1&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v51_0&quot;>
   &lt;soapenv:Header>
      &lt;univ:SupportedVersions urVersion=&quot;ac vinclis&quot; airVersion=&quot;ac vinclis&quot; hotelVersion=&quot;pectore flammas&quot; vehicleVersion=&quot;pectore flammas&quot; passiveVersion=&quot;annos bella gero&quot; railVersion=&quot;certo et&quot; cruiseVersion=&quot;rates evertitque aequora&quot;/>
      &lt;ses:SessionContext>
         &lt;ses:SessTok id=&quot;claustra fremunt&quot;/>
         &lt;ses:SessProp nm=&quot;imperio premit&quot; val=&quot;quisquam numen&quot;/>
      &lt;/ses:SessionContext>
   &lt;/soapenv:Header>
   &lt;soapenv:Body>
      &lt;univ:UniversalRecordRetrieveReq TraceId=&quot;gero et&quot; TokenId=&quot;sonoras imperio&quot; AuthorizedBy=&quot;quae divum incedo&quot; TargetBranch=&quot;verrantque per auras&quot; OverrideLogging=&quot;ERROR&quot; LanguageCode=&quot;turbine corripuit&quot; RetrieveProviderReservationDetails=&quot;false&quot; ViewOnlyInd=&quot;false&quot; TravelerLastName=&quot;regemque dedit&quot; TravelerFirstName=&quot;fremunt celsa&quot; ReturnUnmaskedData=&quot;false&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;profundum quippe ferant&quot; CIDBNumber=&quot;100&quot;/>
         &lt;com:AgentIDOverride SupplierCode=&quot;et ca&quot; ProviderCode=&quot;iovis&quot; AgentID=&quot;speluncis abdidit&quot;/>
         &lt;com:TerminalSessionInfo>bella gero et&lt;/com:TerminalSessionInfo>
         &lt;com:OverridePCC ProviderCode=&quot;flamm&quot; PseudoCityCode=&quot;hoc metuen&quot;/>
         &lt;univ:UniversalRecordLocatorCode>ac vincl&lt;/univ:UniversalRecordLocatorCode>
         &lt;univ:ProviderReservationInfo ProviderCode=&quot;spelu&quot; ProviderLocatorCode=&quot;aris imponet ho&quot; SupplierCode=&quot;praet&quot;/>
      &lt;/univ:UniversalRecordRetrieveReq>
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
