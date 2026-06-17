//! Ziatora, the Incinerator — `{3}{B}{R}{G}` 6/6 legendary Demon Dragon
//!  with Flying.
//! "At the beginning of your end step, you may sacrifice another
//!  creature. When you do, Ziatora deals damage equal to that creature's
//!  power to any target and you create three Treasure tokens."
//!  The sacrifice and the three Treasures are modeled; the reflexive
//!  "deals damage equal to that creature's power to any target" is GAP'd
//!  — the sacrificed creature's power isn't knowable in the resolver and
//!  the reflexive any-target can't be wired off the sacrifice choice.

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ziatora, the Incinerator");
    let demon = reg.interner_mut().intern("Demon");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: end_step,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn end_step(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: reflexive "When you do, Ziatora deals damage equal to that
    // creature's power to any target" — the sacrificed creature's power
    // can't be read in this resolver and the reflexive any-target can't
    // be attached. Sacrifice + Treasure creation are modeled.
    vec![Effect::Sequence(vec![
        Effect::Sacrifice {
            player: trig.controller,
            filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            count: 1,
        },
        Effect::CreateCommodityToken {
            controller: trig.controller,
            kind: CommodityToken::Treasure,
            count: 3,
        },
    ])]
}
