//! Tawnos, Urza's Apprentice — `{U}{R}` 1/3 Legendary Human Artificer with Haste.
//! "{U}{R}, {T}: Copy target activated or triggered ability you control from an
//!  artifact source. You may choose new targets for the copy."
//!
//! Haste is expressible. The activation cost (mana + tap) is expressible, but
//! "copy target activated or triggered ability" has no TargetFilter for an
//! ability on the stack and no copy-an-ability Effect (CopySpell copies a
//! spell, not an ability), so the targeting and effect are GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tawnos, Urza's Apprentice");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}{R}, {T}: Copy target activated or triggered ability you control from an artifact source. You may choose new targets for the copy.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}{R}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                // GAP: no TargetFilter for an activated/triggered ability on
                // the stack.
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: copy_ability,
            }),
    )
}

fn copy_ability(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "copy target activated or triggered ability" — no copy-an-ability
    // Effect (CopySpell copies a spell, not an ability).
    Vec::new()
}
