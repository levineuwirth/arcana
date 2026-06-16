//! Onakke Oathkeeper — `{1}{W}` 0/4 Ogre Spirit.
//! "Creatures can't attack planeswalkers you control unless their controller pays
//! {1} for each creature they control that's attacking a planeswalker you control.
//! {4}{W}{W}, Exile this card from your graveyard: Return target planeswalker card
//! from your graveyard to the battlefield."
//!
//! The attack-restriction static is GAP'd (no attack-tax-on-planeswalkers
//! representation). The graveyard-activated reanimation ability is expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Onakke Oathkeeper");
    let ogre = reg.interner_mut().intern("Ogre");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre);
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "Creatures can't attack planeswalkers you control unless their controller
    //      pays {1} for each ..." — attack tax on planeswalkers not representable.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{4}{W}{W}, Exile this card from your graveyard: Return target planeswalker card from your graveyard to the battlefield.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{4}{W}{W}").expect("valid cost"),
                exile_self: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::new().with_types(TypeLine::PLANESWALKER.into()),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Graveyard,
            is_instant_speed: false,
            face_gate: None,
            effect: reanimate_planeswalker,
        }),
    )
}

fn reanimate_planeswalker(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}
