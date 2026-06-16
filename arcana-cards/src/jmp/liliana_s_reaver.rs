//! Liliana's Reaver — `{2}{B}{B}` 4/3 Zombie with Deathtouch.
//! "Whenever this creature deals combat damage to a player, that player
//! discards a card and you create a tapped 2/2 black Zombie creature
//! token."

use arcana_core::effects::{Effect, DiscardChoice, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Liliana's Reaver");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    // Restrict the damage source to THIS creature by name.
    let source_filter = ObjectFilter {
        name: Some(name),
        ..ObjectFilter::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter,
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: discard_and_token,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn discard_and_token(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let Some(player) = trig.damaged_player() else {
        return Vec::new();
    };
    let zombie = reg.interner().lookup("Zombie").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    vec![
        // "that player discards a card" — the discarding player chooses
        // their own card to discard.
        Effect::Discard {
            player,
            count: 1,
            choice: DiscardChoice::OpponentChooses,
        },
        // GAP: the token is created TAPPED; there is no demonstrated
        // create-tapped token primitive, so it enters untapped.
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: zombie,
                colors: ColorSet::black(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(2)),
                toughness: Some(PtValue::Fixed(2)),
                keywords: vec![],
                abilities: vec![],
            },
        },
    ]
}
