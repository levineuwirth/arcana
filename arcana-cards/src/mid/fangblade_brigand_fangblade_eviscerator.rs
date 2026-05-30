//! Fangblade Brigand // Fangblade Eviscerator — `{3}{R}` 3/4 red Human Werewolf.
//!
//! Front face (Fangblade Brigand):
//!   {1}{R}: This creature gets +1/+0 and gains first strike until end of turn.
//!   Daybound (GAP: day/night cycle not modeled.)
//!
//! Back face (Fangblade Eviscerator):
//!   {1}{R}: This creature gets +1/+0 and gains first strike until end of turn.
//!   {4}{R}: Creatures you control get +2/+0 until end of turn.
//!   Nightbound (GAP: day/night cycle not modeled.)
//!
//! GAP: Daybound/Nightbound keywords not in KeywordAbility enum.
//! GAP: Back-face-only activated ability ({4}{R}: pump all creatures) not auto-installed.
//! GAP: Back-face-only triggered ability not modeled.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fangblade Brigand");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        // GAP: Daybound keyword not in KeywordAbility enum
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Fangblade Eviscerator");
    let back_werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_werewolf_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![],
            // GAP: Nightbound keyword not in KeywordAbility enum
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // {1}{R}: +1/+0 and first strike until end of turn (front face)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}: This creature gets +1/+0 and gains first strike until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None, // shared on both faces
                effect: pump_first_strike,
            })
            // GAP: back-face-only {4}{R}: creatures you control get +2/+0 until end of turn
            // not modeled (back-face-only activated ability not auto-installed on transform)
    )
}

fn pump_first_strike(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::FirstStrike],
    }]
}
