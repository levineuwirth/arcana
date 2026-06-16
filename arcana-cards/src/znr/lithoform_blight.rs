//! Lithoform Blight — `{1}{B}` enchantment — Aura.
//! "Enchant land. When this Aura enters, draw a card. Enchanted land loses
//!  all land types and abilities and has '{T}: Add {C}' and '{T}, Pay 1
//!  life: Add one mana of any color.'"
//!
//! Enchant land via a land-typed permanent filter. The ETB "draw a card"
//! is expressible (DrawCards). The "loses all land types and abilities and
//! has [these two mana abilities]" grant has no expressible ability-removal
//! + mana-ability-grant builder — GAP for that clause.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lithoform Blight");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Permanent(
                ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
            ))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "enchanted land loses all land types and abilities and has
    // '{T}: Add {C}' and '{T}, Pay 1 life: Add one mana of any color'" —
    // no ability-removal + granted-mana-ability builder. Only the ETB draw
    // is expressible.
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}
