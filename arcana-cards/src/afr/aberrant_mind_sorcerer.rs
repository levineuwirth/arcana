//! Aberrant Mind Sorcerer — `{4}{U}` 3/4 Human Elf Shaman Sorcerer.
//!
//! Oracle:
//! * Psionic Spells — "When this creature enters, choose target instant or
//!   sorcery card in your graveyard, then roll a d20. 1—9 | You may put
//!   that card on top of your library. 10—20 | Return that card to your
//!   hand." — an ETB trigger that chooses an instant-or-sorcery card in
//!   your graveyard. The d20 roll and its two outcome buckets are not
//!   expressible (no dice-roll Effect), so the resolution body is GAP'd;
//!   the target choice is kept faithful.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aberrant Mind Sorcerer");
    let human = reg.interner_mut().intern("Human");
    let elf = reg.interner_mut().intern("Elf");
    let shaman = reg.interner_mut().intern("Shaman");
    let sorcerer = reg.interner_mut().intern("Sorcerer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(elf);
    subtypes.0.insert(shaman);
    subtypes.0.insert(sorcerer);

    let is_filter =
        ObjectFilter::new().with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY));

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: psionic_spells,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: is_filter,
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn psionic_spells(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "roll a d20; 1—9 you may put that card on top of your library,
    // 10—20 return it to your hand" — no dice-roll Effect and no way to
    // dispatch on the roll bucket; the choose-target step is kept above.
    Vec::new()
}
