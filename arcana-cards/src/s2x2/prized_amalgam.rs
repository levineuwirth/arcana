//! Prized Amalgam — `{1}{U}{B}` 3/3 blue-black Zombie. "Whenever a creature
//! enters, if it entered from your graveyard or you cast it from your
//! graveyard, return this card from your graveyard to the battlefield tapped
//! at the beginning of the next end step."
//! GAP: trigger condition — no variant for "whenever a creature enters from
//! your graveyard or is cast from graveyard". Closest is ZoneChange from
//! graveyard to battlefield; GAP: no way to also include cast-from-graveyard.
//! GAP: effect — return this card from graveyard at beginning of next end step
//! (delayed trigger firing in the graveyard zone). Use zone: vec![Zone::Graveyard(0)].

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Prized Amalgam");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — should fire when a creature enters from graveyard
                // OR is cast from graveyard; using ZoneChange graveyard→battlefield
                // as approximation.
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Graveyard(0)),
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: on_gy_creature_etb,
                trigger_zones: vec![Zone::Graveyard(0)],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_gy_creature_etb(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no Effect variant for "return this card from graveyard to
    // battlefield tapped at beginning of next end step".
    // ReturnFromGraveyardToBattlefield doesn't model the tapped-delayed form.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: trig.source }]
}
