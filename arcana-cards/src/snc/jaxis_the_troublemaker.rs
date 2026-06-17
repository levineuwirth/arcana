//! Jaxis, the Troublemaker — `{3}{R}` 2/3 Legendary Human Warrior.
//! "{R}, {T}, Discard a card: Create a token that's a copy of another
//!  target creature you control. It gains haste and 'When this token dies,
//!  draw a card.' Sacrifice it at the beginning of the next end step.
//!  Activate only as a sorcery."
//!  (the token copy is wired; the haste / dies-draw / end-step-sacrifice
//!   riders on the minted token are GAP'd — CopyPermanent mints a plain copy
//!   with no rider hook.)
//! Blitz {1}{R} — alternative cast cost, GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jaxis, the Troublemaker");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Blitz {1}{R} — alternative cast cost.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{R}, {T}, Discard a card: Create a token that's a copy of another target creature you control. It gains haste and \"When this token dies, draw a card.\" Sacrifice it at the beginning of the next end step. Activate only as a sorcery.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{R}").expect("valid cost"),
                tap: true,
                discard_other: Some(ObjectFilter::default()),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: make_copy,
        }),
    )
}

fn make_copy(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::CopyPermanent { target: *id }]
}
