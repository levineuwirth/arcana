//! Memory Vampire — `{4}{U}{B}` 4/4 Creature — Vampire Detective. U/B.
//!
//! Oracle:
//! - Flying — keyword. ("Collect evidence" and "Mill" are ability words /
//!   non-keyword reminder tags, not usable KeywordAbility variants.)
//! - "Whenever this creature deals combat damage to a player, any number of
//!   target players each mill that many cards. Then you may collect evidence 9.
//!   When you do, you may cast target nonland card from defending player's
//!   graveyard without paying its mana cost." — combat-damage-to-player
//!   trigger; each chosen target player mills a number of cards equal to the
//!   combat damage dealt.
//!   GAP: "collect evidence 9" and the reflexive "cast target nonland card …
//!   without paying its mana cost" rider are not expressible.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Memory Vampire");
    let vampire = reg.interner_mut().intern("Vampire");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(detective);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: arcana_core::targets::ObjectFilter::default(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: mill_target_players,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Any,
                    controller: None,
                }],
            }),
    )
}

fn mill_target_players(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = trig.damage_amount().unwrap_or(0);
    if n == 0 {
        return Vec::new();
    }
    let mills: Vec<Effect> = trig
        .targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Player(p) => Some(Effect::Mill { player: *p, count: n }),
            _ => None,
        })
        .collect();
    // GAP: "Then you may collect evidence 9. When you do, you may cast target
    // nonland card from defending player's graveyard without paying its mana
    // cost." — collect-evidence + free-cast rider not expressible.
    if mills.is_empty() {
        Vec::new()
    } else {
        vec![Effect::Sequence(mills)]
    }
}
