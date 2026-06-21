//! Mica, Reader of Ruins — `{3}{R}` 4/4 Legendary Human Artificer.
//!
//! Oracle:
//! * Ward—Pay 3 life.
//! * Whenever you cast an instant or sorcery spell, you may sacrifice an
//!   artifact. If you do, copy that spell and you may choose new targets for
//!   the copy.
//!
//! Ward—Pay 3 life is a NON-MANA Ward cost, which is not expressible
//! (`KeywordAbility::Ward` takes a `ManaCost`), so `keywords` is empty and the
//! ability is GAP'd. The spell-cast trigger is wired with its condition, but
//! its effect is GAP'd: the "you may sacrifice an artifact; if you do, copy
//! that spell" gate is an optional-SACRIFICE payment (OptionalPayment only
//! supports Mana/Life) feeding a copy of the triggering spell — not
//! expressible with the available primitives.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mica, Reader of Ruins");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: Ward—Pay 3 life is a non-mana Ward cost; not expressible.
        keywords: vec![],
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
                effect: maybe_sac_and_copy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn maybe_sac_and_copy(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: optional sacrifice-an-artifact payment (OptionalPayment supports
    // only Mana/Life) gating a copy of the triggering spell with new targets.
    Vec::new()
}
