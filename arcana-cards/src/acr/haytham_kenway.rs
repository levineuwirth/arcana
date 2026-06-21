//! Haytham Kenway — `{2}{W}{U}` 3/3 Legendary Creature — Human Knight.
//!
//! * "Protection from Assassins" — Protection is not an expressible
//!   `KeywordAbility`; GAP'd (keywords empty).
//! * "Other Knights you control get +2/+2 and have protection from Assassins."
//!   — a static anthem with no trigger word or cost; not a triggered/activated
//!   ability, so GAP'd.
//! * "When Haytham Kenway enters, for each opponent, exile up to one target
//!   creature that player controls until Haytham Kenway leaves the
//!   battlefield." — emitted as an ETB O-ring (`ExileUntilSourceLeaves`)
//!   exiling up to one opponent-controlled creature. The "for each opponent"
//!   fan-out (one target per opponent) is a multiplayer fidelity gap; one
//!   target is offered.

use arcana_core::effects::Effect;
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Haytham Kenway");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };
    // GAP: "Protection from Assassins" keyword + the "Other Knights you control
    // get +2/+2 and have protection from Assassins" static anthem.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_exile,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_exile(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "for each opponent, exile up to one" — multiplayer fan-out not
    // modeled; exiles up to one opponent creature.
    trig.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::ExileUntilSourceLeaves {
                source: trig.source,
                target: *id,
            }),
            _ => None,
        })
        .collect()
}
