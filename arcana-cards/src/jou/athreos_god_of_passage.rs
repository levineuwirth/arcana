//! Athreos, God of Passage — `{1}{W}{B}` 5/4 Legendary Enchantment Creature — God.
//! Indestructible.
//! As long as your devotion to white and black is less than seven, Athreos isn't
//! a creature. (static — GAP, no devotion-gated type-removal Effect.)
//! Whenever another creature you own dies, return it to your hand unless target
//! opponent pays 3 life.

use arcana_core::effects::Effect;
use arcana_core::actions::OptionalPaymentKind;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::effects::KeywordAbility;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Athreos, God of Passage");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);

    // GAP: "As long as your devotion to white and black is less than seven, Athreos
    // isn't a creature." — devotion-gated type removal is not an expressible static.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Indestructible],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: on_creature_dies,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Player,
                count: TargetCount::Exactly(1),
                controller: Some(ControllerConstraint::Opponent),
            }],
        }),
    )
}

fn on_creature_dies(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.dying_object() else { return Vec::new(); };
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let chooser = match target {
        TargetChoice::Player(p) => *p,
        TargetChoice::Object(_) => return Vec::new(),
        TargetChoice::ObjectOrPlayer(_) => return Vec::new(),
    };
    // "return it to your hand UNLESS target opponent pays 3 life":
    // the opponent may pay 3 life to avoid the return.
    vec![Effect::OptionalPayment {
        chooser,
        cost: OptionalPaymentKind::Life(3),
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: Some(Box::new(Effect::ReturnFromGraveyardToHand { target: id })),
    }]
}
