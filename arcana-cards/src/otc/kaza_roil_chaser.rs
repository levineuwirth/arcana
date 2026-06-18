//! Kaza, Roil Chaser — `{U}{R}` 1/2 Legendary Human Wizard with Flying
//! and Haste.
//! "{T}: The next instant or sorcery spell you cast this turn costs {X}
//! less to cast, where X is the number of Wizards you control as this
//! ability resolves."
//!
//! Flying + Haste are base keywords. The tap ability's payload — a
//! next-spell dynamic cost reduction — has no expressible primitive
//! (no next-cast cost-reduction effect), so its effect is GAP'd while
//! the tap activation shape is kept.

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
    let name = reg.interner_mut().intern("Kaza, Roil Chaser");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: The next instant or sorcery spell you cast this turn costs {X} less to cast, where X is the number of Wizards you control as this ability resolves.".into(),
            cost: ActivationCost::tap_only(),
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: cost_reduction,
        }),
    )
}

fn cost_reduction(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "the next instant or sorcery spell you cast this turn costs {X}
    // less" — no next-cast cost-reduction primitive in the documented API.
    Vec::new()
}
