//! Ganax, Astral Hunter — `{4}{R}` 3/4 Legendary Dragon.
//!
//! Flying
//! Whenever Ganax or another Dragon you control enters, create a
//! Treasure token.
//! Choose a Background. (Deckbuilding rule — GAP'd.)
//!
//! GAP: keyword line `Choose a background` / `Treasure` are not usable
//! `KeywordAbility` variants — `keywords` is just `Flying`. The "Choose
//! a Background" line is a Commander deckbuilding permission with no
//! board effect, so it is GAP'd (no `Effect`/ability expresses it).

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ganax, Astral Hunter");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let dragon_filter = script::subtype_filter(reg, "Dragon")
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            // "Ganax OR another Dragon you control enters" — a Dragon
            // ETB ZoneChange covers both Ganax itself and the others.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: dragon_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: create_treasure,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_treasure(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Treasure,
        count: 1,
    }]
}

// GAP unused import guard not needed; ObjectFilter is used by dragon_filter.
#[allow(unused_imports)]
use arcana_core::targets::ObjectFilter as _ObjectFilterUsed;
