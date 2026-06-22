//! Dragonback Lancer — `{3}{W}` 3/3 Creature — Human Soldier.
//!
//! Oracle:
//! * Flying
//! * Mobilize 1 (Whenever this creature attacks, create a tapped and
//!   attacking 1/1 red Warrior creature token. Sacrifice it at the
//!   beginning of the next end step.)
//!
//! Flying is a base keyword. Mobilize 1 has no `KeywordAbility` variant,
//! so it is decomposed into a SelfAttacks trigger creating a 1/1 red
//! Warrior token via `CreateTokenSacEot` (created + sacrificed at the
//! next end step) — mirroring the catalog's Nightblade Brigade. Fidelity
//! gap: the token isn't created tapped and attacking.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dragonback Lancer");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let _warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        // GAP: Mobilize keyword line — no KeywordAbility::Mobilize
        // (modeled as the attacks trigger below).
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: mobilize_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn mobilize_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let warrior = match reg.interner().lookup("Warrior") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(warrior);
    vec![Effect::CreateTokenSacEot {
        controller: trig.controller,
        token: TokenDefinition {
            name: warrior,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
