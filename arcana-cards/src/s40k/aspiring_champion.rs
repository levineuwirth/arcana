//! Aspiring Champion — `{3}{R}` 3/3 red Astartes Warrior.
//! Menace.
//! Ruinous Ascension — When this creature deals combat damage to a player,
//! sacrifice it. If you do, reveal cards from the top of your library until you
//! reveal a creature card. Put that card onto the battlefield, then shuffle the
//! rest into your library. If that creature is a Demon, it deals damage equal
//! to its power to each opponent.
//!
//! Menace is a base keyword. The combat-damage trigger sacrifices this creature
//! (matched by name) and reveals-until a creature card to put it onto the
//! battlefield. The "If you do" linkage between the sacrifice and the reveal is
//! not enforced (both effects run), and the Demon power-damage rider is GAP'd
//! (no way to gate on the put creature's subtype / read its power post-resolve).

use arcana_core::effects::{Effect, KeywordAbility, RevealDest, DigRest};
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
    let name = reg.interner_mut().intern("Aspiring Champion");
    let astartes = reg.interner_mut().intern("Astartes");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(astartes);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::new(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: ruinous_ascension,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn ruinous_ascension(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "If that creature is a Demon, it deals damage equal to its power to
    //       each opponent" — no way to gate on the put creature's subtype nor
    //       read its power after RevealUntil resolves.
    let self_name = reg.interner().lookup("Aspiring Champion");
    vec![Effect::Sequence(vec![
        Effect::Sacrifice {
            player: trig.controller,
            filter: ObjectFilter {
                name: self_name,
                ..ObjectFilter::default()
            },
            count: 1,
        },
        Effect::RevealUntil {
            player: trig.controller,
            filter: ObjectFilter::creature(),
            found_dest: RevealDest::Battlefield,
            rest: DigRest::BottomRandom,
            max_reveal: None,
        },
    ])]
}
