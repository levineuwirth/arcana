//! Ulamog's Reclaimer — `{4}{U}` 2/5 colorless Eldrazi Processor (Devoid).
//!
//! Oracle:
//! * Devoid — the card has no color (recorded as `ColorSet::colorless()`;
//!   Devoid is not a `KeywordAbility` variant, it is expressed by the
//!   colorless color set).
//! * "When this creature enters, you may put a card an opponent owns from
//!   exile into that player's graveyard. If you do, return target instant
//!   or sorcery card from your graveyard to your hand." — ETB trigger.
//!
//! The Processor cost (moving an opponent-owned card from exile into
//! their graveyard) is not in the documented effect surface — that "if
//! you do" gate is GAP'd. The visible payload, returning a targeted
//! instant or sorcery card from your graveyard to your hand, IS
//! expressible and is wired as the trigger's effect.

use arcana_core::effects::Effect;
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ulamog's Reclaimer");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let processor = reg.interner_mut().intern("Processor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(processor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_return_instant_or_sorcery,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::new()
                        .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn etb_return_instant_or_sorcery(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Processor cost "you may put a card an opponent owns from exile
    //      into that player's graveyard" + the "if you do" gate are not in
    //      the documented effect surface; the targeted graveyard return
    //      (the visible payload) is emitted unconditionally.
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ReturnFromGraveyardToHand { target: *id }]
}
