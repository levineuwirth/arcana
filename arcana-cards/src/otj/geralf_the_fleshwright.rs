//! Geralf, the Fleshwright — `{2}{U}` 2/3 Legendary Human Warlock.
//! "Whenever you cast a spell during your turn other than your first
//! spell that turn, create a 2/2 blue and black Zombie Rogue creature
//! token."
//! "Whenever a Zombie you control enters, put a +1/+1 counter on it for
//! each other Zombie that entered the battlefield under your control this
//! turn."
//!
//! Both triggers are wired. The first uses a custom intervening-if (your
//! turn AND at least two of your spells cast this turn, i.e. not the
//! first). The second scales the +1/+1 counters off the number of Zombies
//! that entered under your control this turn (minus the one entering).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PlayerId, PtValue, SubtypeSet, SupertypeSet,
    TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Geralf, the Fleshwright");
    let human = reg.interner_mut().intern("Human");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    let zombie_filter = script::subtype_filter(reg, "Zombie")
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: Some(if_not_first_spell_your_turn),
                effect: make_zombie_rogue,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: zombie_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: counter_per_other_zombie,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_not_first_spell_your_turn(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    // "during your turn ... other than your first spell that turn" — the
    // just-cast spell is already logged when the trigger goes on the
    // stack, so the 2nd, 3rd, ... spell makes this count >= 2.
    if s.active_player() != you {
        return false;
    }
    script::spells_cast_this_turn(s, &ObjectFilter::default(), you) >= 2
}

fn make_zombie_rogue(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let zombie = reg.interner().lookup("Zombie").unwrap_or_default();
    let rogue = reg.interner().lookup("Rogue").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(rogue);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: zombie,
            colors: ColorSet::blue() | ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn counter_per_other_zombie(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let id = trig.entering_object().unwrap_or(trig.source);
    let zombie_filter = script::subtype_filter(reg, "Zombie")
        .controlled_by(ControllerConstraint::You);
    // "for each OTHER Zombie that entered under your control this turn" —
    // subtract the one currently entering (it is included in the count).
    let entered = script::entered_this_turn_matching(state, &zombie_filter, trig.controller);
    let n = entered.saturating_sub(1);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::AddCounters {
        target: id,
        kind: CounterKind::PlusOnePlusOne,
        count: n,
    }]
}
