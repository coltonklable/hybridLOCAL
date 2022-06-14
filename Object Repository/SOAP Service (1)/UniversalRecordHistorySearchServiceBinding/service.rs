<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>315ae5d6-8d3f-43a6-9372-9a4426348060</elementGuidId>
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
      <value>http://localhost:8080/kestrel/UniversalRecordHistorySearchService</value>
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
      &lt;univ:SupportedVersions urVersion=&quot;claustra fremunt&quot; airVersion=&quot;imperio premit&quot; hotelVersion=&quot;quisquam numen&quot; vehicleVersion=&quot;ac vinclis&quot; passiveVersion=&quot;ac vinclis&quot; railVersion=&quot;pectore flammas&quot; cruiseVersion=&quot;pectore flammas&quot;/>
   &lt;/soapenv:Header>
   &lt;soapenv:Body>
      &lt;univ:UniversalRecordHistorySearchReq TraceId=&quot;gero et&quot; TokenId=&quot;sonoras imperio&quot; AuthorizedBy=&quot;quae divum incedo&quot; TargetBranch=&quot;verrantque per auras&quot; OverrideLogging=&quot;ERROR&quot; LanguageCode=&quot;turbine corripuit&quot; RetrieveProviderReservationDetails=&quot;false&quot; UniversalRecordLocatorCode=&quot;regemque&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;fremunt celsa&quot; CIDBNumber=&quot;100&quot;/>
         &lt;com:AgentIDOverride SupplierCode=&quot;feran&quot; ProviderCode=&quot;scept&quot; AgentID=&quot;turbine corripuit scopuloque&quot;/>
         &lt;com:TerminalSessionInfo>flammato secum dea&lt;/com:TerminalSessionInfo>
         &lt;com:OverridePCC ProviderCode=&quot;profu&quot; PseudoCityCode=&quot;nubibus ig&quot;/>
         &lt;univ:UniversalRecordHistorySearchModifiers MaxResults=&quot;20&quot; StartFromResult=&quot;1&quot; ModifiedBy=&quot;anySimpleType&quot; DebugMode=&quot;false&quot;>
            &lt;univ:ElementType>ac vinclis et&lt;/univ:ElementType>
            &lt;univ:ModifiedDate>2005-09-12-06:00&lt;/univ:ModifiedDate>
            &lt;univ:ModifiedRange>
               &lt;univ:ModifiedStart>2006-11-29&lt;/univ:ModifiedStart>
               &lt;univ:ModifiedEnd>2009-11-03-07:00&lt;/univ:ModifiedEnd>
            &lt;/univ:ModifiedRange>
         &lt;/univ:UniversalRecordHistorySearchModifiers>
      &lt;/univ:UniversalRecordHistorySearchReq>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://localhost:8080/kestrel/UniversalRecordHistorySearchService</soapServiceEndpoint>
   <soapServiceFunction>service</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <wsdlAddress>file:/S:/TestData/UAPI_JAVA_Framework/wsdl/universal_v51_0/UniversalRecord.wsdl</wsdlAddress>
</WebServiceRequestEntity>
