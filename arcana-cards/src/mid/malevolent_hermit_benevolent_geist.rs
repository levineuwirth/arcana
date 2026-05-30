//! Malevolent Hermit // Benevolent Geist — `{1}{U}` Human Wizard 2/1 (front).
//!
//! Front face:
//! - {U}, Sacrifice this creature: Counter target noncreature spell unless
//!   its controller pays {3}.
//! - Disturb {2}{U}: not in usable keyword surface; omitted.
//!
//! Back face (Benevolent Geist): Flying 2/1 Spirit Wizard.
//! - Noncreature spells you control can't be countered. (static ability —
//!   GAP: "can't be countered" static replacement not modeled.)
//! - If Benevolent Geist would be put into a graveyard from anywhere, exile
//!   it instead. (GAP: graveyard-replacement not modeled.)
//!
//! GAP: Disturb keyword not in the usable keyword surface; omitted.
//! GAP: Back-face static "can't be countered" not modeled.
//! GAP: Back-face graveyard-to-exile replacement effect not modeled.
//! GAP: back-face-only triggered ability not modeled.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivationContext, ActivationCost, ActivationZone, ActivatedAbilityDef,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Malevolent Hermit");
    let human_sub = reg.interner_mut().intern("Human");
    let wizard_sub = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(wizard_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // Back face: Benevolent Geist — Flying 2/1 Spirit Wizard.
    let back_name = reg.interner_mut().intern("Benevolent Geist");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let wizard_sub2 = reg.interner_mut().intern("Wizard");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(spirit_sub);
    back_subtypes.0.insert(wizard_sub2);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet::default(),
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    // {U}, Sacrifice this creature: Counter target noncreature spell
    // unless its controller pays {3}.
    let noncreature_spell_req = TargetRequirement {
        filter: TargetFilter::Spell(
            ObjectFilter::new().without_types(TypeLine::CREATURE.into()),
        ),
        count: TargetCount::Exactly(1),
        controller: None,
    };

    let counter_ability = ActivatedAbilityDef {
        text: "{U}, Sacrifice this creature: Counter target noncreature spell unless its controller pays {3}.".into(),
        cost: ActivationCost {
            mana_cost: ManaCost::parse("{U}").expect("valid cost"),
            sacrifice: true,
            ..ActivationCost::default()
        },
        target_requirements: vec![noncreature_spell_req],
        is_mana_ability: false,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: true,
        face_gate: None,
        effect: counter_unless_pays,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_activated_ability(counter_ability),
    )
}

fn counter_unless_pays(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::CounterUnlessPays {
        target: *id,
        cost: ManaCost::parse("{3}").expect("valid cost"),
    }]
}
