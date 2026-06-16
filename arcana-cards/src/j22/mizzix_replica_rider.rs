//! Mizzix, Replica Rider — `{4}{R}` 4/5 Legendary Goblin Wizard with Flying.
//! "Whenever you cast a spell from anywhere other than your hand, you may
//! pay {1}{U/R}. If you do, copy that spell and you may choose new targets
//! for the copy. If the copy is a permanent spell, it gains haste and 'At
//! the beginning of your end step, sacrifice this permanent.'"
//! (GAP — the "from anywhere other than your hand" zone restriction on a
//! cast trigger is unexpressible, and there is no accessor to obtain the
//! triggering spell's stack id to feed Effect::CopySpell.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mizzix, Replica Rider");
    let goblin = reg.interner_mut().intern("Goblin");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: cannot restrict SpellCast to "from anywhere other than
            // your hand"; using any spell you cast. Body is GAP'd because the
            // triggering spell's stack id is not an exposed accessor.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: copy_that_spell,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn copy_that_spell(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may pay {1}{U/R}; if you do, copy that spell …" — no
    // accessor for the triggering spell's stack object id, so CopySpell
    // cannot be targeted, and the permanent-token/haste/sac rider is
    // likewise inexpressible.
    Vec::new()
}
