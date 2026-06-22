//! Clay Champion — `{X}{4}` Artifact Creature — Construct, 2/2 (colorless).
//!
//! Oracle:
//! * This creature enters with three +1/+1 counters on it for each {G}{G} spent
//!   to cast it.
//! * When this creature enters, choose up to two other target creatures you
//!   control. For each {W}{W} spent to cast this creature, put a +1/+1 counter
//!   on each of them.
//!
//! Both abilities scale on the colored mana SPENT to cast the spell, which is
//! not inspectable with the demonstrated API (cf. Adamant / Sunburst). The
//! enters-with clause is a mana-spent replacement (no trigger) and is GAP'd
//! entirely. The ETB "choose up to two other target creatures" is emitted as a
//! targeted trigger, but its counter payload (per-{W}{W}-spent) is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Clay Champion");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "enters with three +1/+1 counters for each {G}{G} spent to cast it" —
    // a mana-spent-dependent enters-with replacement; the colored mana spent to
    // cast a spell is not inspectable with the demonstrated API.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
            }),
    )
}

fn etb_counters(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "for each {W}{W} spent to cast this creature, put a +1/+1 counter on
    // each of the chosen creatures." The counter count scales on the colored
    // mana SPENT to cast the spell, which is not inspectable with the
    // demonstrated API. The chosen targets are validated, but the payload is
    // unexpressible.
    Vec::new()
}
