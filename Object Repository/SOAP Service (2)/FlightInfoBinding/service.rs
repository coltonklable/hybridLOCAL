<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>01d2eec2-6259-4348-bea5-8a4cd2abcbd0</elementGuidId>
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
      <value>http://localhost:8080/kestrel/FlightService</value>
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
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:air=&quot;http://www.travelport.com/schema/air_v49_0&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v49_0&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;air:FlightInformationReq TraceId=&quot;gero et&quot; TokenId=&quot;sonoras imperio&quot; AuthorizedBy=&quot;quae divum incedo&quot; TargetBranch=&quot;verrantque per auras&quot; OverrideLogging=&quot;ERROR&quot; LanguageCode=&quot;turbine corripuit&quot; RetrieveProviderReservationDetails=&quot;false&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;regemque dedit&quot; CIDBNumber=&quot;100&quot;/>
         &lt;com:AgentIDOverride SupplierCode=&quot;fremu&quot; ProviderCode=&quot;feran&quot; AgentID=&quot;sceptra tenens&quot;/>
         &lt;com:TerminalSessionInfo>turbine corripuit scopuloque&lt;/com:TerminalSessionInfo>
         &lt;com:OverridePCC ProviderCode=&quot;flamm&quot; PseudoCityCode=&quot;profundum &quot;/>
         &lt;air:FlightInfoCriteria Key=&quot;nubibus ignem disiecitque&quot; Carrier=&quot;ac&quot; FlightNumber=&quot;rapid&quot; Origin=&quot;cae&quot; Destination=&quot;cir&quot; DepartureDate=&quot;2009-11-03+05:30&quot; ClassOfService=&quot;cl&quot;/>
      &lt;/air:FlightInformationReq>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://localhost:8080/kestrel/FlightService</soapServiceEndpoint>
   <soapServiceFunction>service</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <wsdlAddress>file:/C:/Users/ramraja.sampathkumar/Desktop/SVT/POC_REST_SOAP/UAPI/wsdl/air_v49_0/Air.wsdl</wsdlAddress>
</WebServiceRequestEntity>
