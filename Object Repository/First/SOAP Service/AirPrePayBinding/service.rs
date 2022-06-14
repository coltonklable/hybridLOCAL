<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>35264a29-e0df-4091-a21b-edc2d3fa1258</elementGuidId>
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
      &lt;air:AirPrePayReq TraceId=&quot;gero et&quot; TokenId=&quot;sonoras imperio&quot; AuthorizedBy=&quot;quae divum incedo&quot; TargetBranch=&quot;verrantque per auras&quot; OverrideLogging=&quot;ERROR&quot; LanguageCode=&quot;turbine corripuit&quot; RetrieveProviderReservationDetails=&quot;false&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;regemque dedit&quot; CIDBNumber=&quot;100&quot;/>
         &lt;com:AgentIDOverride SupplierCode=&quot;fremu&quot; ProviderCode=&quot;feran&quot; AgentID=&quot;sceptra tenens&quot;/>
         &lt;com:TerminalSessionInfo>turbine corripuit scopuloque&lt;/com:TerminalSessionInfo>
         &lt;com:OverridePCC ProviderCode=&quot;flamm&quot; PseudoCityCode=&quot;profundum &quot;/>
         &lt;air:ListSearch StartFromResult=&quot;100&quot; MaxResults=&quot;100&quot;>
            &lt;air:PersonNameSearch Last=&quot;nubibus ignem disiecitque&quot;/>
            &lt;com:LoyaltyCard Key=&quot;ac vinclis et&quot; SupplierCode=&quot;ra&quot; AllianceLevel=&quot;caelumque profundum&quot; MembershipProgram=&quot;circum claustra&quot; ElStat=&quot;A&quot; KeyOverride=&quot;true&quot; CardNumber=&quot;arce sceptra&quot; Status=&quot;circum claustra fremunt&quot; MembershipStatus=&quot;certo et premere&quot; FreeText=&quot;et montis&quot; SupplierType=&quot;Air&quot; Level=&quot;ac vinclis&quot; PriorityCode=&quot;pectore flammas&quot; VendorLocationRef=&quot;pectore flammas&quot;>
               &lt;com:ProviderReservationSpecificInfo ProviderReservationLevel=&quot;true&quot; ReservationLevel=&quot;true&quot;>
                  &lt;com:OperatedBy>certo et&lt;/com:OperatedBy>
                  &lt;com:ProviderReservationInfoRef Key=&quot;rates evertitque aequora&quot;/>
               &lt;/com:ProviderReservationSpecificInfo>
            &lt;/com:LoyaltyCard>
         &lt;/air:ListSearch>
         &lt;air:PrePayRetrieve Id=&quot;volutans nimborum in&quot; Type=&quot;faciat maria&quot;/>
      &lt;/air:AirPrePayReq>
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
