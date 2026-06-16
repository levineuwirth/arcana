//! Sisay's Ingenuity — `{U}` enchantment — Aura.
//! "Enchant creature. When this Aura enters, draw a card. Enchanted creature
//!  has '{2}{U}: Target creature becomes the color of your choice until end
//!  of turn.'"
//!
//! The ETB "draw a card" rides the fixed SelfEntersBattlefield trigger. The
//! granted host activated ability ("becomes the color of your choice") needs
//! a chosen-color target on a granted ability — not expressible with the
//! demonstrated host-activated surface — GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sisay's Ingenuity");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: granted host activated ability "{2}{U}: target creature becomes the
    // color of your choice until end of turn" — chosen-color target on a
    // granted ability is not expressible.
    let Some(you) = _state.objects.get(trig.source).map(|o| o.controller) else {
        return Vec::new();
    };
    vec![Effect::DrawCards {
        player: you,
        count: 1,
    }]
}
