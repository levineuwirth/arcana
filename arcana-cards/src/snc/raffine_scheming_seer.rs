//! Raffine, Scheming Seer — `{W}{U}{B}` 1/4 Legendary Sphinx Demon.
//! Flying, ward {1}.
//! Whenever you attack, target attacking creature connives X, where X is the
//! number of attacking creatures. (Connive X = draw X, discard X; +1/+1 counter
//! per nonland discarded.)
//!
//! Flying and Ward {1} are wired. The attack trigger uses the closest
//! demonstrated condition (`CreatureAttacks` over your creatures, once per
//! turn), targets an attacking creature, and connives X where X is the number
//! of attacking creatures (computed with `script::count_matching` over an
//! attacking-only filter): draw X, discard X.
//!
//! GAP: Connive is not in the demonstrated keyword surface, and the connive
//! "put a +1/+1 counter on that creature per nonland card discarded" rider
//! depends on the identity of the just-discarded cards, which is not an
//! observable resolution-time amount — omitted (the draw-X/discard-X core is
//! wired). The once-per-turn frequency approximates "whenever you attack" (the
//! batched attack event).

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Raffine, Scheming Seer");
    let sphinx = reg.interner_mut().intern("Sphinx");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sphinx);
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Ward(ManaCost::parse("{1}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CreatureAttacks {
                filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            },
            intervening_if: None,
            effect: connive_x,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::OncePerTurn,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(ObjectFilter::creature().attacking_only()),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn connive_x(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let x = script::count_matching(
        state,
        &ObjectFilter::creature().attacking_only(),
        trig.controller,
    );
    if x == 0 {
        return Vec::new();
    }
    vec![
        Effect::DrawCards { player: trig.controller, count: x },
        Effect::Discard {
            player: trig.controller,
            count: x,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
