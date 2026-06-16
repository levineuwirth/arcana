//! Anowon, the Ruin Sage — `{3}{B}{B}` 4/3 black Legendary Vampire Shaman.
//! "At the beginning of your upkeep, each player sacrifices a non-Vampire
//! creature." The non-Vampire sacrifice filter is expressed with
//! `ObjectFilter::without_subtype_sym`.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Anowon, the Ruin Sage");
    let vampire = reg.interner_mut().intern("Vampire");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: each_player_sacrifices_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn each_player_sacrifices_creature(
    state: &GameState,
    _trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Non-Vampire creature ("Vampire" interned in register; on a failed
    // lookup skip the exclusion — never-interned subtype is on no object).
    let mut non_vampire = ObjectFilter::creature();
    if let Some(vampire) = reg.interner().lookup("Vampire") {
        non_vampire = non_vampire.without_subtype_sym(vampire);
    }
    let players = script::all_players(state);
    let effects: Vec<Effect> = players.into_iter().map(|p| {
        Effect::Sacrifice {
            player: p,
            filter: non_vampire.clone(),
            count: 1,
        }
    }).collect();
    vec![Effect::Sequence(effects)]
}
