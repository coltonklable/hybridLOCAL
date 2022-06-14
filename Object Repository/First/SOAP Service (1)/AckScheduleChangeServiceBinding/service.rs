<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>68b6f36d-8ff0-4660-b55d-d161cf32c416</elementGuidId>
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
      <value>http://localhost:8080/kestrel/ScheduleChangeService</value>
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
      &lt;univ:SupportedVersions urVersion=&quot;circum claustra&quot; airVersion=&quot;coniunx una&quot; hotelVersion=&quot;arce sceptra&quot; vehicleVersion=&quot;circum claustra fremunt&quot; passiveVersion=&quot;certo et premere&quot; railVersion=&quot;et montis&quot; cruiseVersion=&quot;feta furentibus&quot;/>
   &lt;/soapenv:Header>
   &lt;soapenv:Body>
      &lt;univ:AckScheduleChangeReq TraceId=&quot;gero et&quot; TokenId=&quot;sonoras imperio&quot; AuthorizedBy=&quot;quae divum incedo&quot; TargetBranch=&quot;verrantque per auras&quot; OverrideLogging=&quot;ERROR&quot; LanguageCode=&quot;turbine corripuit&quot; RetrieveProviderReservationDetails=&quot;false&quot; UniversalRecordLocatorCode=&quot;regemque dedit&quot; ProviderCode=&quot;fremu&quot; ProviderLocatorCode=&quot;ferant rapidi&quot; ReservationLocatorCode=&quot;sceptra tenens&quot; Version=&quot;100&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;turbine corripuit scopuloque&quot; CIDBNumber=&quot;100&quot;/>
         &lt;com:AgentIDOverride SupplierCode=&quot;flamm&quot; ProviderCode=&quot;profu&quot; AgentID=&quot;nubibus ignem disiecitque&quot;/>
         &lt;com:TerminalSessionInfo>ac vinclis et&lt;/com:TerminalSessionInfo>
         &lt;com:OverridePCC ProviderCode=&quot;rapid&quot; PseudoCityCode=&quot;caelumque &quot;/>
      &lt;/univ:AckScheduleChangeReq>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://localhost:8080/kestrel/ScheduleChangeService</soapServiceEndpoint>
   <soapServiceFunction>service</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <wsdlAddress>file:/S:/TestData/UAPI_JAVA_Framework/wsdl/universal_v51_0/UniversalRecord.wsdl</wsdlAddress>
</WebServiceRequestEntity>
