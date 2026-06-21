//! Shiko, Paragon of the Way — `{2}{U}{R}{W}` 4/5 Legendary Spirit Dragon.
//! Flying, vigilance.
//! When Shiko enters, exile target nonland card with mana value 3 or less
//! from your graveyard. Copy it, then you may cast the copy without paying
//! its mana cost.
//!
//! Keywords are base characteristics. The ETB targets a nonland graveyard
//! card with mv ≤ 3 and exiles it; the "copy it, then you may cast the copy
//! without paying its mana cost" rider is not expressible with the demonstrated
//! effect surface (no copy-a-graveyard-card-and-free-cast primitive), so the
//! resolver exiles the targeted card and GAPs the copy-cast.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::effects::KeywordAbility;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shiko, Paragon of the Way");
    let spirit = reg.interner_mut().intern("Spirit");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(dragon);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_exile_copy_cast,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::new()
                        .without_types(TypeLine::LAND.into())
                        .with_max_cmc(3)
                        .controlled_by(ControllerConstraint::You),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn etb_exile_copy_cast(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "Copy it, then you may cast the copy without paying its mana cost"
    // — no copy-a-graveyard-card-and-free-cast primitive. Exiling the targeted
    // card is the faithful portion.
    vec![Effect::ExileFromGraveyard { target: *id }]
}
