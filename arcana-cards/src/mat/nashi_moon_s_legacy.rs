//! Nashi, Moon's Legacy — `{B}{G}{U}` 3/4 Legendary Rat Shaman.
//!
//! Menace, ward {1}
//! Whenever Nashi attacks, exile up to one target legendary or Rat card from
//! your graveyard and copy it. You may cast the copy.
//!
//! The attack trigger targets up to one legendary card in a graveyard and
//! exiles it. The "or Rat" half of the target filter (supertype-OR-subtype)
//! and the "copy it, you may cast the copy" payoff have no expressible
//! primitive (no copy-a-graveyard-card-and-cast), so those are documented
//! GAPs; the exile portion is faithful.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nashi, Moon's Legacy");
    let rat = reg.interner_mut().intern("Rat");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{G}{U}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![
            KeywordAbility::Menace,
            KeywordAbility::Ward(ManaCost::parse("{1}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: exile_from_graveyard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                // GAP: "or Rat" half of the filter (supertype-OR-subtype) — the
                // requirement only constrains to legendary cards.
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new()
                            .with_supertypes(SupertypeSet(SupertypeSet::LEGENDARY)),
                    },
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn exile_from_graveyard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "copy it. You may cast the copy." — no copy-a-graveyard-card-and-
    // cast primitive; only the exile is performed.
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ExileFromGraveyard { target: *id }]
}
