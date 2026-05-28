//! Spurnmage Advocate — `{W}` 1/1 white Human Nomad.
//! "{T}: Return two target cards from an opponent's graveyard to their hand.
//! Destroy target attacking creature."
//!
//! GAP: This ability has three targets (two cards from opponent's GY + one
//! attacking creature). The two graveyard cards returned + creature destroyed
//! all in one activation is complex but expressible with multiple targets.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spurnmage Advocate");
    let human = reg.interner_mut().intern("Human");
    let nomad = reg.interner_mut().intern("Nomad");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(nomad);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Return two target cards from an opponent's graveyard to their hand. Destroy target attacking creature.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Card {
                            zone: Zone::Graveyard(0),
                            filter: ObjectFilter::new().controlled_by(ControllerConstraint::Opponent),
                        },
                        count: TargetCount::Exactly(2),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                ],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: advocate_effect,
            }),
    )
}

fn advocate_effect(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    // Return first two cards from opponent graveyard
    for i in 0..2 {
        if let Some(t) = ctx.targets.targets.get(i) {
            if let TargetChoice::Object(id) = t {
                effects.push(Effect::ReturnFromGraveyardToHand { target: *id });
            }
        }
    }
    // Destroy the attacking creature (third target)
    if let Some(t) = ctx.targets.targets.get(2) {
        if let TargetChoice::Object(id) = t {
            effects.push(Effect::DestroyPermanent { target: *id });
        }
    }
    effects
}
