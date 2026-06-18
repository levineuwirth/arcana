//! Owen Grady, Raptor Trainer — `{1}{R}{G}` 3/2 Legendary Human Soldier
//! Scientist.
//! "Partner with Blue, Loyal Raptor. {T}: Put your choice of a menace,
//! trample, reach, or haste counter on target Dinosaur. Activate only
//! as a sorcery."
//!
//! Partner / Partner with are not in the usable keyword surface (GAP'd
//! from the keyword line). The activated ability's tap cost +
//! sorcery-speed + target-Dinosaur shape is kept; the payload (choosing
//! among keyword counters) has no expressible CounterKind variant for
//! menace/trample/reach/haste keyword counters, so the effect is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Owen Grady, Raptor Trainer");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let scientist = reg.interner_mut().intern("Scientist");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    subtypes.0.insert(scientist);

    let dino_filter = script::subtype_filter(reg, "Dinosaur");

    // GAP: Partner with Blue, Loyal Raptor / Partner — not in the usable
    // KeywordAbility surface.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Put your choice of a menace, trample, reach, or haste counter on target Dinosaur. Activate only as a sorcery.".into(),
            cost: ActivationCost {
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(dino_filter),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: put_keyword_counter,
        }),
    )
}

fn put_keyword_counter(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "put your choice of a menace, trample, reach, or haste counter"
    // — keyword counters are not modeled in CounterKind; no chooser-among-
    // keyword-counters primitive.
    Vec::new()
}
