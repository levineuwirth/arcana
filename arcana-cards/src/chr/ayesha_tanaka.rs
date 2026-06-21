//! Ayesha Tanaka — `{W}{W}{U}{U}` 2/2 Legendary Human Artificer with Banding.
//!
//! Banding.
//! {T}: Counter target activated ability from an artifact source unless that
//! ability's controller pays {W}. (Mana abilities can't be targeted.)
//!
//! GAP: "counter target activated ability" — there is no `Effect` to counter
//! an activated ability on the stack, and `TargetFilter` has no
//! activated-ability-on-stack variant (only `Spell(ObjectFilter)` for spells).
//! The tap-activated ability is wired with no target and an empty effect; its
//! counter-an-ability payload is the gap.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ayesha Tanaka");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{W}{U}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Banding],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Counter target activated ability from an artifact source unless that ability's controller pays {W}."
                .into(),
            cost: ActivationCost {
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: counter_artifact_ability,
        }),
    )
}

/// GAP: countering a targeted activated ability is not expressible (see module
/// doc).
fn counter_artifact_ability(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    Vec::new()
}
