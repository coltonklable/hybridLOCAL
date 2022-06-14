<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AirPrice Request</name>
   <tag></tag>
   <elementGuidId>97b33528-b5ee-4e15-b504-01dd516209eb</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic dUFQSTkwNDQxOTkwNzctZjMyYWM2MTM6TmV3cGFzc180VSoyMA==</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soap:Envelope xmlns:soap=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
	&lt;soap:Body>
		&lt;air:AirPriceReq AuthorizedBy=&quot;PANDAY&quot; TargetBranch=&quot;P7113930&quot; FareRuleType=&quot;none&quot; xmlns:air=&quot;http://www.travelport.com/schema/air_v50_0&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v50_0&quot;>
			&lt;com:BillingPointOfSaleInfo OriginApplication=&quot;UAPI&quot;/>
			&lt;air:AirItinerary>
				  &lt;air:AirSegment Key=&quot;REfX7wtY1DKAbLrDAAAAAA==&quot; Group=&quot;0&quot; ProviderCode=&quot;1G&quot; Carrier=&quot;QF&quot; FlightNumber=&quot;419&quot; Origin=&quot;SYD&quot; Destination=&quot;MEL&quot; DepartureTime=&quot;2021-12-16&quot; ArrivalTime=&quot;2021-12-16&quot; Distance=&quot;456&quot; ETicketability=&quot;Yes&quot; Equipment=&quot;73H&quot; ChangeOfPlane=&quot;false&quot; ParticipantLevel=&quot;Secure Sell&quot; LinkAvailability=&quot;true&quot; PolledAvailabilityOption=&quot;Cached status used. Polled avail exists&quot; OptionalServicesIndicator=&quot;false&quot; AvailabilitySource=&quot;Q&quot; AvailabilityDisplayType=&quot;Fare Shop/Optimal Shop&quot;>&lt;/air:AirSegment>
          
      
			&lt;/air:AirItinerary>
			&lt;com:SearchPassenger Code=&quot;ADT&quot; BookingTravelerRef=&quot;1&quot;/>
			&lt;com:SearchPassenger Code=&quot;CHD&quot; BookingTravelerRef=&quot;2&quot;/>
			&lt;com:SearchPassenger Code=&quot;INF&quot; BookingTravelerRef=&quot;3&quot;/>
			&lt;air:AirPricingCommand>
			&lt;/air:AirPricingCommand>
		&lt;/air:AirPriceReq>
	&lt;/soap:Body>
&lt;/soap:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://WAS-UNIVERSALAPI-33890-100.UT.TVLPORT.COM:33890/kestrel/AirService</soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
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
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
