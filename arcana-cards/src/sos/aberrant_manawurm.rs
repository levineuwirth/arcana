//! Aberrant Manawurm — `{3}{G}` 2/5 Wurm with Trample.
//! "Whenever you cast an instant or sorcery spell, this creature gets
//! +X/+0 until end of turn, where X is the amount of mana spent to
//! cast that spell."
//!
//! Trample is a base characteristic. The cast-trigger is wired
//! (SpellCast, instant-or-sorcery, you), but its pump amount X is the
//! mana spent to cast the spell — there is no PendingTrigger accessor
//! for "mana spent", so the dynamic amount is uncomputable and the
//! effect is GAP'd (a literal pump would be materially wrong).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aberrant Manawurm");
    let wurm = reg.interner_mut().intern("Wurm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wurm);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new().with_types_any(TypeLine(
                            TypeLine::INSTANT | TypeLine::SORCERY,
                        )),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: pump_by_mana_spent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn pump_by_mana_spent(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "+X/+0, where X is the amount of mana spent to cast that
    // spell" — no PendingTrigger accessor exposes mana spent, so the
    // dynamic amount cannot be computed.
    Vec::new()
}
