//! Adarkar Windform — `{4}{U}` 3/3 Snow Creature — Illusion.
//! Flying.
//! `{1}{S}: Target creature loses flying until end of turn.`
//!
//! The keyword line (Flying) is a base characteristic. The activated
//! ability is wired with its cost and a `target creature` requirement,
//! but the EFFECT is GAP'd: there is no engine primitive for "loses a
//! SINGLE specified keyword (flying)". `Effect::LoseAllAbilities`
//! strips every ability, which is materially different, so the effect
//! body returns `Vec::new()` rather than misrepresent the card.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Adarkar Windform");
    let illusion = reg.interner_mut().intern("Illusion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(illusion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{S}: Target creature loses flying until end of turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{S}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: lose_flying,
        }),
    )
}

fn lose_flying(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "loses flying" — no engine primitive removes a SINGLE named
    // keyword from a target; LoseAllAbilities strips everything, which
    // is materially different.
    Vec::new()
}
