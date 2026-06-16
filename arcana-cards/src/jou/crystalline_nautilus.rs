//! Crystalline Nautilus — `{2}{U}` 4/4 Enchantment Creature — Nautilus.
//! Bestow {3}{U}{U} — NOT in the supported keyword surface (GAP).
//! "When this permanent becomes the target of a spell or ability, sacrifice it."
//! "Enchanted creature gets +4/+4 and has '…'" — bestow Aura static (GAP).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crystalline Nautilus");
    let nautilus = reg.interner_mut().intern("Nautilus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nautilus);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    // GAP: "Bestow {3}{U}{U}" — Bestow is not part of the supported
    // KeywordAbility surface; the alternative Aura cast mode is omitted.
    // GAP: "Enchanted creature gets +4/+4 and has '…'" — the bestow Aura static
    // is a continuous attach-static with no triggered/activated hook; omitted.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfBecomesTarget {
                caster: ControllerConstraint::Any,
            },
            intervening_if: None,
            effect: sacrifice_self,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn sacrifice_self(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "sacrifice it" — no Effect sacrifices a SPECIFIC source object
    // (Effect::Sacrifice picks by filter/count and Effect::DestroyPermanent is
    // a destroy, not a sacrifice); the self-sacrifice is omitted.
    Vec::new()
}
