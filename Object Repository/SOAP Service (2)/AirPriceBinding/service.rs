<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>service</name>
   <tag></tag>
   <elementGuidId>2868f1cc-c903-43d6-b78e-ac038b80ea86</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
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
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic dUFQSTkwNDQxOTkwNzctZjMyYWM2MTM6TmV3cGFzc180VSoyMA==</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soap:Envelope xmlns:soap=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
   &lt;soap:Body>
      &lt;air:AirPriceReq AuthorizedBy=&quot;PANDAY&quot; TargetBranch=&quot;P7113929&quot; FareRuleType=&quot;none&quot; xmlns:air=&quot;http://www.travelport.com/schema/air_v50_0&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v50_0&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;UAPI&quot;/>
         &lt;air:AirItinerary>
            &lt;air:AirSegment Key=&quot;OQOTAvsY1DKAXrePBAAAAA==&quot; Group=&quot;0&quot; Carrier=&quot;AA&quot; ProviderCode=&quot;1G&quot; FlightNumber=&quot;2361&quot; Origin=&quot;DEN&quot; Destination=&quot;ORD&quot; DepartureTime=&quot;2021-12-07T14:12:00.000-07:00&quot; ArrivalTime=&quot;2021-12-07T17:44:00.000-06:00&quot; FlightTime=&quot;152&quot; Distance=&quot;903&quot; ETicketability=&quot;Yes&quot; Equipment=&quot;738&quot; ChangeOfPlane=&quot;false&quot; ParticipantLevel=&quot;Secure Sell&quot; LinkAvailability=&quot;true&quot; PolledAvailabilityOption=&quot;Polled avail used&quot; OptionalServicesIndicator=&quot;false&quot; AvailabilitySource=&quot;S&quot; AvailabilityDisplayType=&quot;Fare Shop/Optimal Shop&quot;>&lt;/air:AirSegment>
         &lt;/air:AirItinerary>
         &lt;com:SearchPassenger Code=&quot;ADT&quot; BookingTravelerRef=&quot;1&quot;/>
         &lt;!--&lt;com:SearchPassenger Code=&quot;ADT&quot; BookingTravelerRef=&quot;2&quot;/>-->
         &lt;!--com:SearchPassenger Code=&quot;CHD&quot; BookingTravelerRef=&quot;2&quot;/-->
         &lt;air:AirPricingCommand/>
      &lt;/air:AirPriceReq>
   &lt;/soap:Body>
&lt;/soap:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://WAS-UNIVERSALAPI-33890-100.UT.TVLPORT.COM:33890/kestrel/AirService</soapServiceEndpoint>
   <soapServiceFunction>service</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress>file:/C:/Users/ramraja.sampathkumar/Desktop/SVT/POC_REST_SOAP/UAPI/wsdl/air_v49_0/Air.wsdl</wsdlAddress>
</WebServiceRequestEntity>
