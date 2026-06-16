//! Kav Landseeker — `{3}{R}` 4/3 Kavu Soldier with Menace.
//!
//! * Menace (keyword).
//! * ETB: "create a Lander token." — a colorless artifact token is minted.
//!   GAP on the Lander's own printed activated ability
//!   ("{2}, {T}, Sacrifice this token: Search your library for a basic land…")
//!   and on the "at the beginning of the end step on your next turn,
//!   sacrifice that token" delayed clause — neither is expressible on a bare
//!   `TokenDefinition` here (Lander is not a wired commodity token).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kav Landseeker");
    let kavu = reg.interner_mut().intern("Kavu");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kavu);
    subtypes.0.insert(soldier);

    let _lander = reg.interner_mut().intern("Lander");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(arcana_core::types::PtValue::Fixed(4)),
        toughness: Some(arcana_core::types::PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_make_lander,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_make_lander(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let lander = reg.interner().lookup("Lander").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lander);
    // GAP: Lander's activated land-search ability and the next-end-step
    // self-sacrifice clause are not expressible on a bare token.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: lander,
            colors: ColorSet::colorless(),
            types: TypeLine::ARTIFACT.into(),
            subtypes,
            power: None,
            toughness: None,
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
