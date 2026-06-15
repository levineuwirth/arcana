//! Traveler's Cloak — `{2}{U}` enchantment — Aura.
//! "Enchant creature. As this Aura enters, choose a land type. When this
//!  Aura enters, draw a card. Enchanted creature has landwalk of the
//!  chosen type."
//!
//! Buff Aura. ETB draws a card for the Aura's controller. The
//! "choose a land type" + "has landwalk of the chosen type" grant is not
//! expressible — landwalk is parametrized by a runtime-chosen subtype
//! with no attached builder — GAP.

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
    let name = reg.interner_mut().intern("Traveler's Cloak");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        // GAP: "choose a land type" + "enchanted creature has landwalk of
        //       the chosen type" — runtime-chosen landwalk subtype grant.
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
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(you) = state.objects.get(trig.source).map(|o| o.controller) else {
        return Vec::new();
    };
    vec![Effect::DrawCards { player: you, count: 1 }]
}
