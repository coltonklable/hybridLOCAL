<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>8c2b40a4-1557-432d-a65a-1fb2c48cb7f3</elementGuidId>
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
      &lt;air:AirRefundQuoteReq TraceId=&quot;gero et&quot; TokenId=&quot;sonoras imperio&quot; AuthorizedBy=&quot;quae divum incedo&quot; TargetBranch=&quot;verrantque per auras&quot; OverrideLogging=&quot;ERROR&quot; LanguageCode=&quot;turbine corripuit&quot; RetrieveProviderReservationDetails=&quot;false&quot; Ignore=&quot;false&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;regemque dedit&quot; CIDBNumber=&quot;100&quot;/>
         &lt;com:AgentIDOverride SupplierCode=&quot;fremu&quot; ProviderCode=&quot;feran&quot; AgentID=&quot;sceptra tenens&quot;/>
         &lt;com:TerminalSessionInfo>turbine corripuit scopuloque&lt;/com:TerminalSessionInfo>
         &lt;com:OverridePCC ProviderCode=&quot;flamm&quot; PseudoCityCode=&quot;profundum &quot;/>
         &lt;com:TicketNumber>nubibus ignem&lt;/com:TicketNumber>
         &lt;air:TCRNumber>ac vinclis et&lt;/air:TCRNumber>
         &lt;air:AirRefundModifiers RefundDate=&quot;rapidum iaculata e&quot; AccountCode=&quot;caelumque profundum&quot; TicketDesignator=&quot;circum claustra&quot;/>
         &lt;com:HostToken Host=&quot;coniu&quot; Key=&quot;anySimpleType&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot;>circum claustra fremunt&lt;/com:HostToken>
         &lt;air:ProviderReservationInfo ProviderCode=&quot;certo&quot; ProviderLocatorCode=&quot;et montis&quot; SupplierCode=&quot;feta &quot;/>
      &lt;/air:AirRefundQuoteReq>
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
