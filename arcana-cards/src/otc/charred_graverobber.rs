//! Charred Graverobber — `{2}{B}` 3/1 Skeleton Mercenary.
//! "When this creature enters, return target outlaw card from your graveyard to
//! your hand." (outlaw = Assassin/Mercenary/Pirate/Rogue/Warlock subtype batch)
//! Escape—{3}{B}{B}, Exile four other cards from your graveyard. (Escape is not
//! an available KeywordAbility — GAP, along with "escapes with a +1/+1 counter")

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Charred Graverobber");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let mercenary = reg.interner_mut().intern("Mercenary");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(skeleton);
    subtypes.0.insert(mercenary);

    // "Outlaw" = Assassin, Mercenary, Pirate, Rogue, or Warlock.
    let outlaw_subtypes = vec![
        reg.interner_mut().intern("Assassin"),
        reg.interner_mut().intern("Mercenary"),
        reg.interner_mut().intern("Pirate"),
        reg.interner_mut().intern("Rogue"),
        reg.interner_mut().intern("Warlock"),
    ];

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Escape is not an available KeywordAbility variant; the escape cast
        // path and "escapes with a +1/+1 counter" rider are unmodeled.
        ..Default::default()
    };

    let outlaw_filter = ObjectFilter::new()
        .with_subtypes_any(outlaw_subtypes)
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: return_outlaw,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: outlaw_filter,
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn return_outlaw(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ReturnFromGraveyardToHand { target: *id }]
}
