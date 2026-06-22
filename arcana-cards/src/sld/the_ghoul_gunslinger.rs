//! The Ghoul, Gunslinger — `{1}{B}{B}` 2/3 Legendary Zombie Mutant Rogue with
//! First strike.
//! Whenever The Ghoul or another nontoken Zombie or Mutant you control dies,
//! target player gets two rad counters. If that player is you, create a Treasure
//! token.
//!
//! GAP: keyword — Scryfall lists "Treasure", which is a token type, not a usable
//! `KeywordAbility` variant; the Treasure creation is wired in the trigger below.
//!
//! GAP (partial effect): "target player gets two rad counters" is not
//! expressible — rad counters are PLAYER counters and `Effect::AddCounters`
//! targets an `ObjectId` only (no player-counter effect in the surface). The
//! "If that player is you, create a Treasure token" rider IS wired (resolution-
//! time check that the chosen target player is the controller).

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Ghoul, Gunslinger");
    let zombie = reg.interner_mut().intern("Zombie");
    let mutant = reg.interner_mut().intern("Mutant");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(mutant);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    // "The Ghoul or another nontoken Zombie or Mutant you control" — a single
    // ZoneChange filter that matches nontoken creatures you control with
    // subtype Zombie or Mutant (which includes The Ghoul itself).
    let zombie_sym = reg.interner_mut().intern("Zombie");
    let mutant_sym = reg.interner_mut().intern("Mutant");
    let dies_filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .nontoken()
        .with_subtypes_any(vec![zombie_sym, mutant_sym]);

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: dies_filter,
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: on_death,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_player()],
        }),
    )
}

fn on_death(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "target player gets two rad counters" — no player-counter effect.
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    if *p == trig.controller {
        vec![Effect::CreateCommodityToken {
            controller: trig.controller,
            kind: CommodityToken::Treasure,
            count: 1,
        }]
    } else {
        Vec::new()
    }
}
